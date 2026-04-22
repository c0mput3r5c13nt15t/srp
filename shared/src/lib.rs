pub mod config;
pub mod logger;

use serde::Deserialize;
use clap::Parser;
use std::net::Ipv4Addr;

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
   pub remote_addr: Ipv4Addr,
   pub remote_port: u16,
   pub endpoint_addr: String,
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
