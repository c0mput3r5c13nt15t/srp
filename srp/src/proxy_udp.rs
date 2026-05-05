use anyhow::Result;
use bytes::BytesMut;
use log::{error, info};
use quinn::Connection;
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::UdpSocket;
use tokio_util::sync::CancellationToken;

pub async fn proxy_udp_stream(
    connection: Connection,
    endpoint_addr: SocketAddr,
    shutdown: CancellationToken,
) -> Result<()> {
    // ONE UDP socket for everything
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(endpoint_addr).await?;

    info!("[udp client] bound and connected to {endpoint_addr}");

    // mappings only (NO sockets here)
    let mut id_to_addr: HashMap<u64, SocketAddr> = HashMap::new();
    let mut addr_to_id: HashMap<SocketAddr, u64> = HashMap::new();
    let mut next_id: u64 = 1;

    let mut buf = vec![0u8; 2048];

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("[udp client] shutdown received");
                connection.close(0u32.into(), b"shutdown".as_ref());
                break;
            }

            _ = connection.closed() => {
                info!("[udp client] QUIC closed");
                break;
            }

            // QUIC -> UDP
            datagram = connection.read_datagram() => {
                let data = match datagram {
                    Ok(d) => d,
                    Err(e) => {
                        error!("[udp client] read_datagram error: {e}");
                        continue;
                    }
                };

                if data.len() < 8 {
                    continue;
                }

                let id = u64::from_be_bytes(data[..8].try_into().unwrap());

                let payload = &data[8..];

                info!("received {:?} from id {}", payload, id);

                if let Err(e) = socket.send(payload).await {
                    error!("[udp client] udp send error: {e}");
                }
            }

            // UDP -> QUIC
            recv = socket.recv_from(&mut buf) => {
                let (len, addr) = match recv {
                    Ok(v) => v,
                    Err(e) => {
                        error!("[udp client] recv error: {e}");
                        continue;
                    }
                };

                let id = match addr_to_id.get(&addr) {
                    Some(id) => *id,
                    _ => {
                        let id = next_id;
                        next_id += 1;

                        addr_to_id.insert(addr, id);
                        id_to_addr.insert(id, addr);

                        id
                    }
                };

                let mut msg = BytesMut::with_capacity(8 + len);
                msg.extend_from_slice(&id.to_be_bytes());
                msg.extend_from_slice(&buf[..len]);

                if let Err(e) = connection.send_datagram(msg.freeze()) {
                    error!("[udp client] send_datagram error: {e}");
                }
            }
        }
    }

    Ok(())
}
