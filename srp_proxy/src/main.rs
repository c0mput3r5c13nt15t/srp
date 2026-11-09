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

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, pkcs8::DecodePrivateKey};
use log::{LevelFilter, debug, info};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::{ClientConfig, Connection, Endpoint};
use shared::{GatewayConfigResponse, Protocol, ProxyConfigRequest};
use std::{
    fs::File,
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    sync::Arc,
};
use tokio::io::AsyncWriteExt;
use tokio::signal;
use tokio_util::sync::CancellationToken;

use shared::{MAX_MSG_SIZE, NONCE_SIZE};

mod certificate_validation;
use certificate_validation::SkipServerVerification;

mod proxy_tcp;
use proxy_tcp::proxy_tcp_stream;

mod proxy_udp;
use proxy_udp::proxy_udp_stream;

async fn configure_gateway(connection: Connection, request: ProxyConfigRequest) -> Result<()> {
    let (mut send, mut recv) = connection.open_bi().await?;

    let request_bytes = serde_json::to_vec(&request)?;

    debug!("sending config request to gateway: '{:?}'", request_bytes);
    send.write_all(&request_bytes).await?;
    send.finish()?;

    let response_bytes = recv.read_to_end(MAX_MSG_SIZE).await?;
    let response: GatewayConfigResponse = serde_json::from_slice(&response_bytes)?;

    debug!("received response {:?} from gateway", response);

    if response.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            response
                .error_message
                .unwrap_or_else(|| "unknown error".to_string())
        ))
    }
}

async fn authenticate_to_gateway(
    connection: Connection,
    signing_key: SigningKey,
    name: String,
) -> Result<()> {
    info!("Trying to authenticate to gateway");
    let (mut send, mut recv) = connection.open_bi().await?;

    //Send Name
    send.write_u32(name.len() as u32).await?;
    send.write_all(String::as_bytes(&name)).await?;

    //Recieve Nonce
    let mut nonce = [0u8; NONCE_SIZE];
    recv.read_exact(&mut nonce).await?;

    //Sign
    let signature: Signature = signing_key.sign(&nonce);

    //Send signature
    let signature_bytes = signature.to_bytes();
    send.write_all(&signature_bytes).await?;
    info!("Signature Sent");

    //Evaluate Gateway Response
    let response_bytes = recv
        .read_to_end(MAX_MSG_SIZE)
        .await
        .map_err(|e| anyhow::anyhow!("Failed reading gateway response: {e:?}"))?;
    let response: GatewayConfigResponse = serde_json::from_slice(&response_bytes)?;
    info!("Got valid response");

    if response.success {
        Ok(())
    } else {
        Err(anyhow::anyhow!(response.error_message.unwrap_or_else(
            || "Authentication to Gateway failed".to_string()
        )))
    }
}

pub async fn run_proxy(
    gateway_socket: SocketAddr,
    app_socket: SocketAddr,
    config_request: ProxyConfigRequest,
    shutdown: CancellationToken,
    signing_key: SigningKey,
    name: String,
) -> Result<()> {
    let mut endpoint = Endpoint::client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))?;

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("crypto provider failed");

    let client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    endpoint.set_default_client_config(client_config);

    info!("connecting to {}", gateway_socket);

    let connection = endpoint.connect(gateway_socket, "localhost")?.await?;

    info!("connected to {}", connection.remote_address());

    authenticate_to_gateway(connection.clone(), signing_key, name).await?;

    info!("successfully authenticated to gateway");
    info!("configuring gateway via config request");

    configure_gateway(connection.clone(), config_request).await?;

    info!("gateway configuration succeeded");

    tokio::select! {
        _ = shutdown.cancelled() => {
            info!("shutdown before proxy start");
            connection.close(0u32.into(), b"shutdown");
        }

        res = async {
            match config_request.protocol {
                Protocol::Udp => {
                    proxy_udp_stream(connection.clone(), app_socket, shutdown.clone()).await
                }
                Protocol::Tcp => {
                    proxy_tcp_stream(connection.clone(), app_socket, shutdown.clone()).await
                }
            }
        } => {
            res?;
        }
    }

    endpoint.wait_idle().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Parse Arguments
    let args = shared::Args::parse_args();
    let stdout_logging_level_filter = match shared::logging::stdout_log_level_filter_from_arguments(
        args.quiet,
        args.verbose,
        args.debug,
    ) {
        Ok(level) => level,
        Err(err) => {
            eprintln!("Error: {err}");
            LevelFilter::Warn
        }
    };
    let config = shared::config::parse_proxy_config(&args.config);

    // Enable logging
    if let Some(logging_config) = config.logging {
        shared::logging::init(
            stdout_logging_level_filter,
            Some(Path::new(&logging_config.log_file)),
            logging_config.log_level,
        )
        .unwrap();
    } else {
        shared::logging::init(stdout_logging_level_filter, None, None).unwrap();
    }

    // Setup networking
    let gateway_socket = SocketAddr::new(
        IpAddr::V4(config.proxy.gateway_addr),
        config.proxy.gateway_port,
    );
    let app_socket = SocketAddr::new(IpAddr::V4(config.proxy.app_addr), config.proxy.app_port);
    let config_request = ProxyConfigRequest {
        expose_addr: config.proxy.expose_addr,
        expose_port: config.proxy.expose_port,
        protocol: config.proxy.protocol,
    };

    //
    let shutdown = CancellationToken::new();
    let shutdown_clone = shutdown.clone();

    //Read private key file
    let mut keyfile =
        File::open(config.proxy.private_key).expect("Could not open private key file");
    let mut keyfile_contents = String::new();
    keyfile.read_to_string(&mut keyfile_contents)?;
    let signing_key: SigningKey =
        SigningKey::from_pkcs8_pem(&keyfile_contents).expect("Invalid pem format in proxy keyfile");

    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        info!("ctrl+c received");
        shutdown_clone.cancel();
    });

    run_proxy(
        gateway_socket,
        app_socket,
        config_request,
        shutdown,
        signing_key,
        config.proxy.name,
    )
    .await?;
    Ok(())
}
