/*
 *  SRP - Secure Reverse Proxy
 *  Copyright (C) 2026  SRP Team
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs;
use std::io;

use crate::GatewayConfig;
use crate::ProxyConfig;

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
