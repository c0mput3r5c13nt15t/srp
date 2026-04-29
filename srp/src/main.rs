use log::{error, info};
use quinn::Connection;
use quinn::crypto::rustls::QuicClientConfig;
use quinn::{ClientConfig, Endpoint};
use shared::{ClientConfigRequest, ServerConfigResponse};
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

mod certificate_validation;

use certificate_validation::SkipServerVerification;

async fn configure_server(
    connection: Connection,
    request: ClientConfigRequest,
) -> anyhow::Result<()> {
    let (mut send, mut recv) = connection.open_bi().await?;

    let request_bytes = serde_json::to_vec(&request)?;
    send.write_all(&request_bytes).await?;
    send.finish()?;

    let received_bytes = recv.read_to_end(usize::MAX).await?;
    let received: ServerConfigResponse = serde_json::from_slice(&received_bytes)?;

    if received.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            received
                .error_message
                .unwrap_or("unknown error".to_string())
        ))
    }
}

async fn handle_stream(
    mut send: quinn::SendStream,
    mut recv: quinn::RecvStream,
    endpoint_addr: SocketAddr,
) {
    let result = async {
        let tcp_stream = TcpStream::connect(endpoint_addr).await?;

        let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();

        let quic_to_tcp = async {
            tokio::io::copy(&mut recv, &mut tcp_write).await?;
            tcp_write.shutdown().await
        };

        let tcp_to_quic = async {
            tokio::io::copy(&mut tcp_read, &mut send).await?;
            send.finish()?;
            Ok::<_, std::io::Error>(())
        };

        tokio::try_join!(quic_to_tcp, tcp_to_quic)?;

        Ok::<(), anyhow::Error>(())
    }
    .await;

    if let Err(e) = result {
        eprintln!("Proxy error: {:?}", e);
    }
}

async fn proxy_tcp_stream(connection: Connection, endpoint_addr: SocketAddr) -> anyhow::Result<()> {
    loop {
        match connection.accept_bi().await {
            Ok((send, recv)) => {
                tokio::spawn(handle_stream(send, recv, endpoint_addr));
            }
            Err(e) => {
                eprintln!("accept_bi failed: {:?}", e);

                // If connection is still alive, continue
                if connection.close_reason().is_none() {
                    continue;
                } else {
                    break Ok(()); // connection actually closed
                }
            }
        }
    }
}

async fn run_client(
    server_socket: SocketAddr,
    endpoint_socket: SocketAddr,
    config_request: ClientConfigRequest,
) -> anyhow::Result<()> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("[client] failed to install crypto provider");

    let client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    endpoint.set_default_client_config(client_config);
    info!("[client] connecting to srp server {}", server_socket);

    // Connect
    let connection = endpoint
        .connect(server_socket, "localhost")
        .unwrap()
        .await
        .unwrap();
    info!("[client] connected: addr={}", connection.remote_address());

    match configure_server(connection.clone(), config_request).await {
        Ok(()) => {
            info!("[client] server configuration succeeded");
        }
        Err(e) => {
            error!("[client] server configuration failed: {:#}", e);
            return Err(anyhow::anyhow!("server configuration failed: {:#}", e));
        }
    }

    proxy_tcp_stream(connection.clone(), endpoint_socket).await?;

    // TODO: Fix error handling and clean connection quit

    // Cleanup
    endpoint.wait_idle().await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();

    let config: shared::ClientConfig = shared::config::parse_client_config(&args.config);

    let server_socket = SocketAddr::new(
        IpAddr::V4(config.client.server_addr),
        config.client.server_port,
    );
    let endpoint_socket = SocketAddr::new(
        IpAddr::V4(config.client.endpoint_addr),
        config.client.endpoint_port,
    );

    let config_request = ClientConfigRequest {
        expose_addr: config.client.expose_addr,
        expose_port: config.client.expose_port,
        protocol: config.client.protocol,
    };

    run_client(server_socket, endpoint_socket, config_request).await?;
    Ok(())
}
