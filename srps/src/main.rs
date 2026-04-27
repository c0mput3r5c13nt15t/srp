use log::info;
use quinn::Connection;
use tokio::signal;
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};

mod configuration;

use configuration::make_server_endpoint;

async fn open_unidirectional_stream(connection: Connection) -> anyhow::Result<()> {
    let mut send = connection.open_uni().await?;
    send.write_all(b"test").await?;
    send.finish()?;
    Ok(())
}

async fn run_server(addr: SocketAddr) {
    let (endpoint, _server_cert) = make_server_endpoint(addr).unwrap();
    info!("[server] listening for srp clients on {}", addr);

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

                                    if let Err(e) = open_unidirectional_stream(connection.clone()).await {
                                        eprintln!("stream error: {:?}", e);
                                    }

                                    // Keep connection alive until peer closes
                                    connection.closed().await;
                                }
                                Err(e) => eprintln!("connection failed: {:?}", e),
                            }
                        });
                    }
                    None => break, // endpoint closed
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();

    let config: shared::ServerConfig = shared::config::parse_server_config(&args.config);

    let addr = SocketAddr::new(IpAddr::V4(config.server.bind_addr), config.server.bind_port);
    run_server(addr).await;
    Ok(())
}
