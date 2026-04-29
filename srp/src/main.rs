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
use tokio::io::copy;
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

async fn proxy_tcp_stream(connection: Connection, endpoint_addr: SocketAddr) -> anyhow::Result<()> {
    loop {
        let (send, recv) = connection.accept_bi().await?;

        tokio::spawn(async move {
            if let Err(e) = async {
                let (mut send, mut recv) = (send, recv);

                let tcp_stream = match TcpStream::connect(endpoint_addr).await {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("TCP connect failed: {:?}", e);

                        let _ = send.reset(0u32.into());

                        return Ok(());
                    }
                };

                let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();

                let quic_to_tcp = copy(&mut recv, &mut tcp_write);
                let tcp_to_quic = copy(&mut tcp_read, &mut send);

                tokio::try_join!(quic_to_tcp, tcp_to_quic)?;

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
            info!("server configuration succeeded");
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
    };

    run_client(server_socket, endpoint_socket, config_request).await?;
    Ok(())
}
