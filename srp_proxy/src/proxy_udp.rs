use anyhow::{Context, Result};
use bytes::BytesMut;
use log::{error, info};
use quinn::Connection;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc};
use tokio_util::sync::CancellationToken;

use shared::proxy_udp::{UDP_BUFFER_SIZE_BYTES, parse_quic_data};

pub async fn proxy_udp_stream(
    connection: Connection,
    endpoint_addr: SocketAddr,
    shutdown: CancellationToken,
) -> Result<()> {
    // One UDP socket per logical client.
    let mut sockets: HashMap<u64, Arc<UdpSocket>> = HashMap::new();

    // Application responses arrive here, tagged with their client ID.
    let (response_tx, mut response_rx) = mpsc::unbounded_channel::<(u64, Vec<u8>)>();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                connection.close(0u32.into(), b"shutdown");
                break;
            }

            _ = connection.closed() => {
                info!("[udp proxy] QUIC connection closed");
                break;
            }

            // QUIC -> application UDP
            datagram = connection.read_datagram() => {
                let data = match datagram {
                    Ok(data) => data,
                    Err(e) => {
                        error!("[udp proxy] QUIC receive error: {e}");
                        break;
                    }
                };

                let Ok((id, payload)) = parse_quic_data(&data) else {
                    error!("[udp proxy] QUIC parse error");
                    break;
                };

                // Create a dedicated UDP socket for this client.
                if let std::collections::hash_map::Entry::Vacant(e) = sockets.entry(id) {
                    let socket = Arc::new(
                        UdpSocket::bind("0.0.0.0:0").await.context("Failed to bind UDP socket")?
                    );

                    socket.connect(endpoint_addr).await.with_context(|| format!("Failed to connect UDP socket to {endpoint_addr}"))?;

                    let recv_socket = Arc::clone(&socket);
                    let tx = response_tx.clone();

                    tokio::spawn(async move {
                        let mut buf = vec![0u8; UDP_BUFFER_SIZE_BYTES];

                        loop {
                            match recv_socket.recv(&mut buf).await {
                                Ok(len) => {
                                    if tx.send((
                                        id,
                                        buf[..len].to_vec()
                                    )).is_err() {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "[udp proxy] client {id} \
                                         receive error: {e}"
                                    );
                                    break;
                                }
                            }
                        }
                    });

                    e.insert(socket);
                    info!("[udp proxy] created UDP socket for {id}");
                }

                if let Some(socket) = sockets.get(&id) && let Err(e) = socket.send(payload).await {
                    error!(
                        "[udp proxy] send to app for {id}: {e}"
                    );
                }
            }

            // Application UDP -> QUIC
            response = response_rx.recv() => {
                let Some((id, payload)) = response else {
                    break;
                };

                let mut msg = BytesMut::with_capacity(8 + payload.len());
                msg.extend_from_slice(&id.to_be_bytes());
                msg.extend_from_slice(&payload);

                if let Err(e) = connection.send_datagram(msg.freeze()) {
                    error!(
                        "[udp proxy] QUIC send for client {id}: {e}"
                    );
                }
            }
        }
    }

    Ok(())
}
