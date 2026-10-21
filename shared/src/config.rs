use std::fs;
use std::io;

use crate::ProxyConfig;
use crate::GatewayConfig;

fn read_file(file_name: &str) -> io::Result<String> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn parse_gateway_config(file_name: &str) -> GatewayConfig {
    let content = read_file(file_name).expect("Error reading config file");
    toml::from_str(&content).expect("Error parsing config file")
}

// TODO: This is wet code
pub fn parse_proxy_config(file_name: &str) -> ProxyConfig {
    let content = read_file(file_name).expect("Error reading config file");
    toml::from_str(&content).expect("Error parsing config file")
}
