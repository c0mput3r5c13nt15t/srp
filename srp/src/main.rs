use log::{info};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::{ClientConfig, Connection, Endpoint};
use shared::{ClientConfigRequest, Protocol, ServerConfigResponse};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::signal;
use tokio_util::sync::CancellationToken;

use shared::MAX_MSG_SIZE;

mod certificate_validation;
use certificate_validation::SkipServerVerification;

mod proxy_tcp;
use proxy_tcp::proxy_tcp_stream;

mod proxy_udp;
use proxy_udp::proxy_udp_stream;

async fn configure_server(
    connection: Connection,
    request: ClientConfigRequest,
) -> anyhow::Result<()> {
    let (mut send, mut recv) = connection.open_bi().await?;

    let request_bytes = serde_json::to_vec(&request)?;
    send.write_all(&request_bytes).await?;
    send.finish()?;

    let response_bytes = recv.read_to_end(MAX_MSG_SIZE).await?;
    let response: ServerConfigResponse = serde_json::from_slice(&response_bytes)?;

    if response.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            response
                .error_message
                .unwrap_or_else(|| "unknown error".to_string())
        ))
    }
}

pub async fn run_client(
    server_socket: SocketAddr,
    endpoint_socket: SocketAddr,
    config_request: ClientConfigRequest,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("crypto provider failed");

    let client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    endpoint.set_default_client_config(client_config);

    info!("connecting to {}", server_socket);

    let connection = endpoint.connect(server_socket, "localhost")?.await?;

    info!("connected to {}", connection.remote_address());

    configure_server(connection.clone(), config_request).await?;

    info!("server configuration succeeded");

    tokio::select! {
        _ = shutdown.cancelled() => {
            info!("shutdown before proxy start");
            connection.close(0u32.into(), b"shutdown");
        }

        res = async {
            match config_request.protocol {
                Protocol::Udp => {
                    proxy_udp_stream(connection.clone(), endpoint_socket, shutdown.clone()).await
                }
                Protocol::Tcp => {
                    proxy_tcp_stream(connection.clone(), endpoint_socket, shutdown.clone()).await
                }
            }
        } => {
            res?;
        }
    }

    endpoint.wait_idle().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();
    let config = shared::config::parse_client_config(&args.config);

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

    let shutdown = CancellationToken::new();
    let shutdown_clone = shutdown.clone();

    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        info!("ctrl+c received");
        shutdown_clone.cancel();
    });

    run_client(server_socket, endpoint_socket, config_request, shutdown).await?;

    Ok(())
}
