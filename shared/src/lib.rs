pub mod config;
pub mod errors;
pub mod logging;
pub mod proxy_udp;

use clap::Parser;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

pub const MAX_CONFIG_SIZE: usize = 64 * 1024;
pub const MAX_MSG_SIZE: usize = 64 * 1024;
pub const NONCE_SIZE: usize = 32;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

// The request of the proxy made to the gateway with details about the config
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ProxyConfigRequest {
    pub expose_addr: Ipv4Addr,
    pub expose_port: u16,
    pub protocol: Protocol,
}

//The request a proxy makes when trying to authenticate
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    pub preshared_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayConfigResponse {
    pub success: bool,
    pub error_message: Option<String>,
}

impl GatewayConfigResponse {
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
pub struct GatewayConfig {
    pub gateway: Gateway,
    pub logging: Option<Logging>,
}

#[derive(Deserialize)]
pub struct Gateway {
    pub bind_addr: Ipv4Addr,
    pub bind_port: u16,
    pub preshared_secret: String,
    // heartbeat_interval: Option<u16>,
}

#[derive(Deserialize)]
pub struct ProxyConfig {
    pub proxy: Proxy,
    pub logging: Option<Logging>,
}

#[derive(Deserialize)]
pub struct Proxy {
    pub gateway_addr: Ipv4Addr,
    pub gateway_port: u16,
    pub app_addr: Ipv4Addr, // TODO: Fix: In Docker containers are often identified by names not IPs -> needs to work with those as well
    pub app_port: u16,
    pub expose_addr: Ipv4Addr,
    pub expose_port: u16,
    pub protocol: Protocol,
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct Logging {
    pub log_file: String,
    pub log_level: Option<LevelFilter>,
}

#[derive(Parser, Debug)]
#[command(name = "srp")]
#[command(about = "Secure reverse proxy for exposing services in private networks.")]
pub struct Args {
    // See https://clig.dev/ for reference
    
    // Config file
    #[arg(short = 'c')]
    #[arg(long = "config")]
    #[arg(value_name = "CONFIG_FILE")]
    #[arg(help = "Config file in toml format")]
    pub config: String,


    // Logging
    // Log by default: ERROR, WARN

    // Only log ERROR
    #[arg(short = 'q')]
    #[arg(long = "quiet")]
    #[arg(help = "Only output errors")]
    pub quiet: bool,

    // Also log INFO
    #[arg(short = 'v')]
    #[arg(long = "verbose")]
    #[arg(help = " ")]
    pub verbose: bool,

    // Also log DEBUG
    #[arg(short = 'd')]
    #[arg(long = "debug")]
    #[arg(help = "Show debugging output")]
    pub debug: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        use clap::Parser; // bring trait into scope internally
        Self::parse()
    }
}
