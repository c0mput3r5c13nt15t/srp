use log::{error, info};
use port_check::is_local_port_free;
use quinn::Connection;
use shared::{AuthRequest, ClientConfigRequest, MAX_CONFIG_SIZE, Protocol, ServerConfigResponse};
use std::net::{IpAddr, SocketAddr};
use tokio::{net::TcpListener, signal, task::JoinSet};
use tokio_util::sync::CancellationToken;

mod configuration;
use configuration::make_server_endpoint;

async fn authenticate_client(
    connection: Connection,
    preshared_secret: &String,
) -> anyhow::Result<()> {
    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .map_err(|e| anyhow::anyhow!("accept bi failed: {e}"))?;

    let bytes = recv
        .read_to_end(MAX_CONFIG_SIZE)
        .await
        .map_err(|e| anyhow::anyhow!("read config failed: {e}"))?;

    let auth_req: AuthRequest = serde_json::from_slice(&bytes)
        .map_err(|e| anyhow::anyhow!("invalid Authentication request JSON: {e}"))?;

    let accept_secret = if &auth_req.preshared_secret == preshared_secret {
        info!(
            "Client {} authenticated successfully",
            connection.remote_address()
        );
        ServerConfigResponse::success()
    } else {
        ServerConfigResponse::error(String::from("Failed to Authenticate Client"))
    };

    let resp_bytes = serde_json::to_vec(&accept_secret)?;

    send.write_all(&resp_bytes).await?;
    send.finish()?;

    if accept_secret.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "client secret rejected: {}",
            accept_secret.error_message.unwrap_or_default()
        ))
    }
}

async fn receive_configuration_from_client(
    connection: Connection,
) -> anyhow::Result<ClientConfigRequest> {
    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .map_err(|e| anyhow::anyhow!("accept bi failed: {e}"))?;

    let bytes = recv
        .read_to_end(MAX_CONFIG_SIZE)
        .await
        .map_err(|e| anyhow::anyhow!("read config failed: {e}"))?;

    let config: ClientConfigRequest =
        serde_json::from_slice(&bytes).map_err(|e| anyhow::anyhow!("invalid config JSON: {e}"))?;

    info!("client config: {:?}", config);

    let response = if !is_local_port_free(config.expose_port) {
        ServerConfigResponse::error(String::from("port already taken"))
    } else {
        ServerConfigResponse::success()
    };

    let resp_bytes = serde_json::to_vec(&response)?;

    send.write_all(&resp_bytes).await?;
    send.finish()?;

    if response.success {
        Ok(config)
    } else {
        Err(anyhow::anyhow!(
            "client config rejected: {}",
            response.error_message.unwrap_or_default()
        ))
    }
}

async fn proxy_tcp_stream(
    connection: Connection,
    listener: TcpListener,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();

    loop {
        tokio::select! {
            // 1. Explicit shutdown (Ctrl+C)
            _ = shutdown.cancelled() => {
                info!("[stream] shutdown received (token)");
                connection.close(0u32.into(), b"shutdown".as_ref());
                break;
            }

            // 2. QUIC connection closed by peer
            _ = connection.closed() => {
                info!("[stream] shutdown received (connection closed)");
                break;
            }

            // 3. Accept TCP connections
            accept = listener.accept() => {
                let (tcp_stream, _) = match accept {
                    Ok(v) => v,
                    Err(e) => {
                        error!("[stream] accept error: {e}");
                        continue;
                    }
                };

                let conn = connection.clone();

                tasks.spawn(async move {
                    let result = async {
                        let (mut send, mut recv) = conn.open_bi().await?;

                        let (mut r, mut w) = tcp_stream.into_split();

                        let up = tokio::io::copy(&mut r, &mut send);
                        let down = tokio::io::copy(&mut recv, &mut w);

                        tokio::try_join!(up, down)?;

                        send.finish()?;
                        Ok::<(), anyhow::Error>(())
                    }
                    .await;

                    if let Err(e) = result {
                        error!("[stream tasks] error: {e:?}");
                    }
                });
            }
        }
    }

    // 4. Drain all active stream tasks
    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            error!("[stream] task join error: {e:?}");
        }
    }

    Ok(())
}

async fn run_server(
    bind_addr: SocketAddr,
    shutdown: CancellationToken,
    preshared_secret: String,
) -> anyhow::Result<()> {
    let (endpoint, _cert) = make_server_endpoint(bind_addr)
        .map_err(|e| anyhow::anyhow!("failed to create endpoint: {e}"))?;

    info!("listening on {bind_addr}");

    let mut tasks = JoinSet::new();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("shutdown received");
                break;
            }

            incoming = endpoint.accept() => {
                let Some(connecting) = incoming else {
                    break;
                };

                let shutdown = shutdown.clone();
                let preshared_secret = preshared_secret.clone();

                tasks.spawn(async move {
                    match connecting.await {
                        Ok(connection) => {
                            info!("connection from {}", connection.remote_address());

                            match authenticate_client(connection.clone(), &preshared_secret).await {
                                Ok(_) => {
                                }
                                Err(e) => {
                                    error!("authentication error: {e}");
                                    return;
                                }
                            }
                            let config = match receive_configuration_from_client(connection.clone()).await {
                                Ok(c) => c,
                                Err(e) => {
                                    error!("config error: {e}");
                                    return;
                                }
                            };

                            let addr = SocketAddr::new(
                                IpAddr::V4(config.expose_addr),
                                config.expose_port,
                            );


                            if config.protocol == Protocol::Tcp {
                                let listener = match TcpListener::bind(addr).await {
                                    Ok(l) => l,
                                    Err(e) => {
                                        error!("bind failed {}: {e}", addr);
                                        return;
                                    }
                                };

                                info!("listening on {}", addr);

                                if let Err(e) = proxy_tcp_stream(
                                    connection,
                                    listener,
                                    shutdown,
                                ).await {
                                    error!("proxy error: {e:?}");
                                }
                            } else {
                                error!("udp tunnels not yet implemented");
                                info!("connection aborted");
                            }
                        }

                        Err(e) => {
                            error!("connection failed: {e}");
                        }
                    }
                });
            }
        }
    }

    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            error!("task error: {e:?}");
        }
    }

    endpoint.wait_idle().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();
    let config = shared::config::parse_server_config(&args.config);

    let bind_addr = SocketAddr::new(IpAddr::V4(config.server.bind_addr), config.server.bind_port);

    let shutdown = CancellationToken::new();
    let shutdown_clone = shutdown.clone();

    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        info!("ctrl+c received");
        shutdown_clone.cancel();
    });

    let preshared_secret = config.server.preshared_secret.clone();

    run_server(bind_addr, shutdown, preshared_secret).await?;
    Ok(())
}
