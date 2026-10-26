use anyhow::Result;
use bytes::BytesMut;
use log::{error, info};
use quinn::Connection;
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::UdpSocket;
use tokio_util::sync::CancellationToken;

use shared::proxy_udp::{UDP_BUFFER_SIZE_BYTES, parse_quic_data};

pub async fn proxy_udp_stream(
    connection: Connection,
    socket: UdpSocket,
    shutdown: CancellationToken,
) -> Result<()> {
    let mut buf = vec![0u8; UDP_BUFFER_SIZE_BYTES];
    // two maps for O(1) access to the id / addr for incoming UDP / QUIC packets
    // TODO: make sure ids don't overflow
    let mut addr_to_id: HashMap<SocketAddr, u64> = HashMap::new();
    let mut id_to_addr: HashMap<u64, SocketAddr> = HashMap::new();
    let mut next_id: u64 = 1;

    info!(
        "[quic] max datagram size: {:?}",
        connection.max_datagram_size()
    );

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("[udp] shutdown received");
                connection.close(0u32.into(), b"shutdown".as_ref());
                break;
            }

            _ = connection.closed() => {
                info!("[udp] connection closed");
                break;
            }

            // UDP -> QUIC
            recv = socket.recv_from(&mut buf) => {
                let (len, addr) = match recv {
                    Ok((len, addr)) => {
                        println!("{:?} bytes received from {:?}", len, addr);
                        (len, addr)
                    },
                    Err(e) => {
                        error!("[udp] recv error: {e}");
                        continue;
                    }
                };

                // Assign or reuse client ID
                let id = match addr_to_id.get(&addr) {
                    Some(&id) => id,
                    None => {
                        let id = next_id;
                        next_id = next_id.checked_add(1)
                            .ok_or_else(|| anyhow::anyhow!("client ID exhausted"))?;

                        addr_to_id.insert(addr, id);
                        id_to_addr.insert(id, addr);
                        id
                    }
                };

                println!("id {:?}", id);

                // Encode: [client_id | payload]
                let mut msg = BytesMut::with_capacity(8 + len);
                msg.extend_from_slice(&id.to_be_bytes());
                msg.extend_from_slice(&buf[..len]);

                if let Err(e) = connection.send_datagram(msg.freeze()) {
                    error!("[udp] send_datagram error: {e}");
                }
            }

            // QUIC -> UDP
            datagram = connection.read_datagram() => {
                let data = match datagram {
                    Ok(d) => d,
                    Err(e) => {
                        error!("[udp] read_datagram error: {e}");
                        continue;
                    }
                };

                let Ok((id, payload)) = parse_quic_data(&data) else {
                    error!("[udp proxy] QUIC parse error");
                    break;
                };

                if let Some(addr) = id_to_addr.get(&id) && let Err(e) = socket.send_to(payload, addr).await {
                    error!("[udp gateway] send error: {e}");
                }
            }
        }
    }

    Ok(())
}
