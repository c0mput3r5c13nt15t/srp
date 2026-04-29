use log::{error, info};
use port_check::is_local_port_free;
use quinn::Connection;
use shared::{ClientConfigRequest, Protocol, ServerConfigResponse};
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};
use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::signal;

mod configuration;

use configuration::make_server_endpoint;

async fn receive_configuration_from_client(
    connection: Connection,
) -> anyhow::Result<ClientConfigRequest> {
    match connection.accept_bi().await {
        Ok((mut send, mut recv)) => {
            let received_bytes = recv.read_to_end(usize::MAX).await?;
            let received: ClientConfigRequest = serde_json::from_slice(&received_bytes)?;

            info!("[server] client provided config: {:?}", received);

            // TODO: is there more config to check?
            // TODO: Handle potential time-of-check to time-of-use (TOCTOU) issues?
            let response = if !is_local_port_free(received.expose_port) {
                ServerConfigResponse::error(String::from("port already taken"))
            } else {
                ServerConfigResponse::success()
            };

            let response_bytes = serde_json::to_vec(&response)?;
            send.write_all(&response_bytes).await?;
            send.finish()?;

            if response.success {
                Ok(received)
            } else {
                Err(anyhow::anyhow!(
                    "[server] client config rejected: {}",
                    response.error_message.unwrap_or_default()
                ))
            }
        }
        Err(e) => Err(e.into()),
    }
}

async fn proxy_tcp_stream(connection: Connection, listener: TcpListener) -> anyhow::Result<()> {
    loop {
        let (tcp_stream, _) = listener.accept().await?;
        let connection = connection.clone();

        tokio::spawn(async move {
            if let Err(e) = async {
                // Open QUIC bidirectional stream
                let (mut send, mut recv) = connection.open_bi().await?;

                let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();

                let tcp_to_quic = copy(&mut tcp_read, &mut send);
                let quic_to_tcp = copy(&mut recv, &mut tcp_write);

                tokio::try_join!(tcp_to_quic, quic_to_tcp)?;

                // Gracefully finish QUIC send side
                send.finish()?;

                Ok::<(), anyhow::Error>(())
            }
            .await
            {
                error!("Proxy error: {:?}", e);
            }
        });
    }
}

// TODO: open_udp_proxy_stream

async fn run_server(bind_addr: SocketAddr) -> anyhow::Result<()> {
    let (endpoint, _server_cert) = match make_server_endpoint(bind_addr) {
        Ok(res) => res,
        Err(e) => {
            error!("[server] error creating QUIC endpoint: {:#}", e);
            return Err(e);
        }
    };
    info!("[server] listening for srp clients on {}", bind_addr);

    loop {
        tokio::select! {
            // Accept incoming connections
            incoming = endpoint.accept() => {
                match incoming {
                    Some(connecting) => {
                        tokio::spawn(async move {
                            match connecting.await {
                                Ok(connection) => {
                                    info!(
                                        "[server] connection accepted: addr={}",
                                        connection.remote_address()
                                    );

                                    let config: ClientConfigRequest = match receive_configuration_from_client(connection.clone()).await {
                                        Ok(config) => {
                                            info!("[server] accepted client config");
                                            config
                                        },
                                        Err(e) => {
                                            error!("{:#}", e);
                                            return;
                                        }
                                    };

                                    let expose_addr = SocketAddr::new(IpAddr::V4(config.expose_addr), config.expose_port);

                                    if config.protocol == Protocol::Tcp {
                                        let listener = match TcpListener::bind(expose_addr).await {
                                            Ok(listener) => {
                                                info!("[server] listening for traffic on {}", expose_addr);
                                                listener
                                            },
                                            Err(e) => {
                                                error!("[server] failed to create tcp listener: {:#}", e);
                                                return;
                                            }
                                        };

                                        if let Err(e) = proxy_tcp_stream(connection.clone(), listener).await {
                                            error!("stream error: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("connection failed: {:?}", e),
                            }
                        });
                    }
                    _none => break, // endpoint closed
                }
            }

            // Handle Ctrl+C
            _ = signal::ctrl_c() => {
                info!("[server] shutting down...");
                break;
            }
        }
    }

    endpoint.wait_idle().await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();

    let config: shared::ServerConfig = shared::config::parse_server_config(&args.config);

    let bind_addr = SocketAddr::new(IpAddr::V4(config.server.bind_addr), config.server.bind_port);
    run_server(bind_addr).await?;
    Ok(())
}
