use log::info;
use quinn::Connection;
use shared::{ClientConfigRequest, ServerConfigResponse};
use tokio::signal;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use tokio::io::{copy};

mod configuration;

use configuration::make_server_endpoint;

async fn configure_client(connection: Connection) -> anyhow::Result<ClientConfigRequest> {
    loop {
        match connection.accept_bi().await {
            Ok((mut send, mut recv)) => {
                let received_bytes = recv.read_to_end(usize::MAX).await?;
                let received: ClientConfigRequest =
                    serde_json::from_slice(&received_bytes)?;

                info!("{}:{}", received.expose_addr, received.expose_port);

                // TODO: check config, e.g. if port is in use

                let response = ServerConfigResponse::error(String::from("address already in use"));

                let response_bytes = serde_json::to_vec(&response)?;
                send.write_all(&response_bytes).await?;
                send.finish()?;

                return Ok(received);
            }

            Err(e) => {
                // connection closed or fatal error
                return Err(e.into());
            }
        }
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
                eprintln!("Proxy error: {:?}", e);
            }
        });
    }
}

// TODO: open_udp_proxy_stream

async fn run_server(bind_addr: SocketAddr) {
    let (endpoint, _server_cert) = make_server_endpoint(bind_addr).unwrap();
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

                                    configure_client(connection.clone()).await.unwrap();

                                    let expose_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080); // TODO: This config should be provided by the client

                                    let listener = TcpListener::bind(expose_addr).await.unwrap();
                                    info!("[server] listening for traffic on {}", expose_addr);

                                    if let Err(e) = proxy_tcp_stream(connection.clone(), listener).await {
                                        eprintln!("stream error: {:?}", e);
                                    }

                                    // Keep connection alive until peer closes
                                    connection.closed().await;
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();

    let config: shared::ServerConfig = shared::config::parse_server_config(&args.config);

    let bind_addr = SocketAddr::new(IpAddr::V4(config.server.bind_addr), config.server.bind_port);
    run_server(bind_addr).await;
    Ok(())
}
