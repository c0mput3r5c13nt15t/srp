use shared;
use log::{info};
use quinn::{Endpoint};
use quinn::ServerConfig;
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};

async fn run_server(addr: SocketAddr) {
    let (endpoint, _server_cert) = make_server_endpoint(addr).unwrap();
    // accept a single connection
    let incoming_conn = endpoint.accept().await.unwrap();
    let conn = incoming_conn.await.unwrap();
    info!(
        "[server] connection accepted: addr={}",
        conn.remote_address()
    );
}

pub(crate) fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_server()
-> Result<(ServerConfig, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
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
