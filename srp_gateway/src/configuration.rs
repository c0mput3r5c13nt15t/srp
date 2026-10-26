use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};

const QUIC_KEEP_ALIVE_INTERVAL_SEC: Duration = Duration::from_secs(10);
const QUIC_DATAGRAM_BUFFER_BYTES: usize = 1_048_576;

pub(crate) fn make_gateway_endpoint(
    bind_addr: SocketAddr,
) -> anyhow::Result<(Endpoint, CertificateDer<'static>)> {
    let (server_config, server_cert) = configure_gateway()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_gateway() -> anyhow::Result<(ServerConfig, CertificateDer<'static>)> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).map_err(|e| {
        anyhow::anyhow!("failed to generate self-signed certificate for localhost: {e}")
    })?;
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).ok_or_else(|| {
        anyhow::anyhow!("failed to get mutable access to QUIC transport configuration")
    })?;
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.keep_alive_interval(Some(QUIC_KEEP_ALIVE_INTERVAL_SEC));

    transport_config.datagram_receive_buffer_size(Some(QUIC_DATAGRAM_BUFFER_BYTES));
    transport_config.datagram_send_buffer_size(QUIC_DATAGRAM_BUFFER_BYTES);

    Ok((server_config, cert_der))
}
