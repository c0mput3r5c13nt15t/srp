use log::{info};
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
};

mod configuration;

use configuration::make_server_endpoint;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    shared::logger::init().unwrap();

    let args = shared::Args::parse_args();

    let config: shared::ServerConfig = shared::config::parse_server_config(&args.config);

    let addr = SocketAddr::new(IpAddr::V4(config.server.bind_addr), config.server.bind_port);
    run_server(addr).await;
    Ok(())
}
