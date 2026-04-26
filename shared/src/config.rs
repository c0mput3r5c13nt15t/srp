use std::io;
use std::fs;

use crate::ServerConfig;
use crate::ClientConfig;

fn read_file(file_name: &str) -> io::Result<String> {
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

pub fn parse_server_config(file_name: &str) -> ServerConfig {
    let content = read_file(file_name).expect("Error reading config file");
    toml::from_str(&content).expect("Error parsing config file")
}

// TODO: This is wet code
pub fn parse_client_config(file_name: &str) -> ClientConfig {
    let content = read_file(file_name).expect("Error reading config file");
    toml::from_str(&content).expect("Error parsing config file")
}
