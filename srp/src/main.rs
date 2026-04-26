use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use log::{info};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::{Endpoint, ClientConfig};

mod certificate_validation;

use certificate_validation::SkipServerVerification;

async fn run_client(server_addr: SocketAddr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    rustls::crypto::ring::default_provider()
    .install_default()
    .expect("failed to install crypto provider");

    let client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    endpoint.set_default_client_config(client_config);

    // Connect
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    info!("[client] connected: addr={}", connection.remote_address());

    // TODO: Send data

    // Drop
    drop(connection);

    // Cleanup
    endpoint.wait_idle().await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();
    
    let args = shared::Args::parse_args();

    let config: shared::ClientConfig = shared::config::parse_client_config(&args.config);

    let addr = SocketAddr::new(IpAddr::V4(config.client.remote_addr), config.client.remote_port);
    run_client(addr).await?;
    Ok(())
}
