pub mod config;
pub mod logger;

use serde::{Serialize, Deserialize};
use clap::Parser;
use std::net::Ipv4Addr;

// The request of the client made to the server with details about the config
#[derive(Serialize, Deserialize)]
pub struct ClientConfigRequest {
   pub expose_addr: Ipv4Addr,
   pub expose_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfigResponse {
   pub success: bool,
   pub error_message: Option<String>,
}

impl ServerConfigResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            error_message: None,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            error_message: Some(msg),
        }
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
   pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
   pub bind_addr: Ipv4Addr,
   pub bind_port: u16
   // heartbeat_interval: Option<u16>,
}

#[derive(Deserialize)]
pub struct ClientConfig {
   pub client: Client,
}

#[derive(Deserialize)]
pub struct Client {
   pub server_addr: Ipv4Addr,
   pub server_port: u16,
   pub endpoint_addr: Ipv4Addr, // Fix: In Docker can be the name of a container
   pub endpoint_port: u16,
   pub expose_addr: Ipv4Addr,
   pub expose_port: u16,
}

#[derive(Parser, Debug)]
#[command(name = "src")]
#[command(about = "Secure reverse proxy for exposing services in private networks.")]
pub struct Args {
    #[arg(short = 'c')]
    #[arg(long = "config")]
    #[arg(value_name = "CONFIG_FILE")]
    #[arg(help = "Config file in toml format")]
    pub config: String,
}

impl Args {
    pub fn parse_args() -> Self {
        use clap::Parser; // bring trait into scope internally
        Self::parse()
    }
}
