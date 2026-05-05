pub mod config;
pub mod logger;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

pub const MAX_CONFIG_SIZE: usize = 64 * 1024;
pub const MAX_MSG_SIZE: usize = 64 * 1024;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

// The request of the client made to the server with details about the config
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ClientConfigRequest {
    pub expose_addr: Ipv4Addr,
    pub expose_port: u16,
    pub protocol: Protocol,
}

//The request a client makes when trying to authenticate
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub preshared_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub bind_port: u16,
    pub preshared_secret: String,
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
    pub endpoint_addr: Ipv4Addr, // TODO: Fix: In Docker containers are often identified by names not IPs -> needs to work with those as well
    pub endpoint_port: u16,
    pub expose_addr: Ipv4Addr,
    pub expose_port: u16,
    pub protocol: Protocol,
    pub preshared_secret: String,
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
