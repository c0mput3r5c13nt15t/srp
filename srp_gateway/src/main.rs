use anyhow::Result;
use log::{LevelFilter, debug, error, info, warn};
use port_check::is_local_port_free;
use quinn::Connection;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Signature, Verifier, VerifyingKey};
use rustls::crypto::aws_lc_rs::sign;
use shared::{AuthRequest, GatewayConfigResponse, MAX_CONFIG_SIZE, MAX_MSG_SIZE, NONCE_SIZE, Protocol, ProxyConfigRequest};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use rand::Rng;
use tokio::{
    net::{TcpListener, UdpSocket},
    signal,
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

mod configuration;
use configuration::make_gateway_endpoint;

mod proxy_tcp;
use proxy_tcp::proxy_tcp_stream;

mod proxy_udp;
use proxy_udp::proxy_udp_stream;

async fn authenticate_proxy(
    connection: Connection,
    public_key: VerifyingKey,
) -> anyhow::Result<()> {
    info!("Starting authentication of proxy");
    let (mut send, mut recv) = connection.open_bi().await?;

    info!("Connection success");
    // Create random Nonce
    // TODO: Critically think about size
    let mut nonce = [0u8;NONCE_SIZE];
    rand::rng().fill_bytes(&mut nonce);

    // Send nonce
    send.write_all(&nonce).await?;
    info!("sent nonce {:?}", nonce);
    
    // Recieve Signature
    let mut signature_bytes =[0u8;SIGNATURE_LENGTH];
    recv.read_exact(&mut signature_bytes).await?;
    let signature:Signature = Signature::try_from(&signature_bytes)?;
    info!("Recieved signature {:?}", signature);

    // Validate Signature
    let accept_secret = if  public_key.verify(&nonce, &signature).is_ok(){
        info!(
            "Proxy {} authenticated successfully",
            connection.remote_address()
        );
        GatewayConfigResponse::success()
    } else {
        GatewayConfigResponse::error(String::from("Failed to Authenticate Proxy"))
    };

    let resp_bytes = serde_json::to_vec(&accept_secret)?;
    info!("Sending response bytes {:?}", resp_bytes);
    send.write_all(&resp_bytes).await?;
    send.finish()?;
    info!("Sent Response to Proxy");

    if accept_secret.success{
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Proxy signature rejected: {}",
            accept_secret.error_message.unwrap_or_default()
        ))
    }
}

async fn receive_configuration_from_proxy(connection: Connection) -> Result<ProxyConfigRequest> {
    let (mut send, mut recv) = connection
        .accept_bi()
        .await
        .map_err(|e| anyhow::anyhow!("accept bi failed: {e}"))?;

    let bytes = recv
        .read_to_end(MAX_CONFIG_SIZE)
        .await
        .map_err(|e| anyhow::anyhow!("read config failed: {e}"))?;

    let config: ProxyConfigRequest =
        serde_json::from_slice(&bytes).map_err(|e| anyhow::anyhow!("invalid config JSON: {e}"))?;

    info!("received config request from proxy: {:?}", config);

    // Run early check of config to return config error before actually trying to connect
    let response = if config.protocol == Protocol::Tcp {
        if is_local_port_free(config.expose_port) == false {
            debug!(
                "pre-check indicates received config request is invalid because the requested port is already taken"
            );
            GatewayConfigResponse::error(String::from("port already taken"))
        } else {
            debug!("pre-check indicates received config request is valid");
            GatewayConfigResponse::success()
        }
    } else {
        // TODO: Check if UDP port is free
        debug!("pre-check indicates received config request is valid");
        GatewayConfigResponse::success()
    };

    let resp_bytes = serde_json::to_vec(&response)?;

    debug!("sending response to config request: '{:?}'", resp_bytes);
    send.write_all(&resp_bytes).await?;
    send.finish()?;

    if response.success {
        Ok(config)
    } else {
        Err(anyhow::anyhow!(response.error_message.unwrap_or_default()))
    }
}

async fn run_gateway(bind_addr: SocketAddr, shutdown: CancellationToken) -> anyhow::Result<()> {
    let (endpoint, _cert) = make_gateway_endpoint(bind_addr)
        .map_err(|e| anyhow::anyhow!("failed to create endpoint: {e}"))?;

    info!("listening for proxies on {}/quic", bind_addr);

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
                //TODO: Get from Config
                let verifying_key_raw: &[u8;PUBLIC_KEY_LENGTH] = &[00;32];
                let verifying_key:VerifyingKey = VerifyingKey::from_bytes(verifying_key_raw)?;

                tasks.spawn(async move {
                    match connecting.await {
                        Ok(connection) => {
                            info!("connection from {}", connection.remote_address());

                            match authenticate_proxy(connection.clone(), verifying_key).await {
                                Ok(_) => {
                                }
                                Err(e) => {
                                    error!("authentication error: {e}");
                                    return;
                                }
                            }
                            let config = match receive_configuration_from_proxy(connection.clone()).await {
                                Ok(c) => c,
                                Err(e) => {
                                    warn!("config request from proxy was rejected: {e}");
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

                                info!("listening on {}/tcp", addr);

                                if let Err(e) = proxy_tcp_stream(
                                    connection,
                                    listener,
                                    shutdown,
                                ).await {
                                    error!("tcp proxy error: {e:?}");
                                }
                            } else {
                                let listener = match UdpSocket::bind(addr).await {
                                    Ok(l) => l,
                                    Err(e) => {
                                        error!("bind failed {}: {e}", addr);
                                        return;
                                    }
                                };

                                info!("listening on {}/udp", addr);

                                if let Err(e) = proxy_udp_stream(
                                    connection,
                                    listener,
                                    shutdown,
                                ).await {
                                    error!("udp proxy error: {e:?}");
                                }
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
    // Parse arguments
    let args = shared::Args::parse_args();
    let stdout_logging_level_filter = match shared::logging::stdout_log_level_filter_from_arguments(
        args.quiet,
        args.verbose,
        args.debug,
    ) {
        Ok(level) => level,
        Err(err) => {
            eprintln!("Error: {err}");
            LevelFilter::Warn
        }
    };
    let config = shared::config::parse_gateway_config(&args.config);

    // Enable logging
    if let Some(logging_config) = config.logging {
        shared::logging::init(
            stdout_logging_level_filter,
            Some(Path::new(&logging_config.log_file)),
            logging_config.log_level,
        )
        .unwrap();
    } else {
        shared::logging::init(stdout_logging_level_filter, None, None).unwrap();
    }

    // Setup networking
    let bind_socket = SocketAddr::new(
        IpAddr::V4(config.gateway.bind_addr),
        config.gateway.bind_port,
    );

    //
    let shutdown = CancellationToken::new();
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        info!("ctrl+c received");
        shutdown_clone.cancel();
    });

    run_gateway(bind_socket, shutdown).await?;
    Ok(())
}
