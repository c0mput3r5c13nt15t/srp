use quinn::{Endpoint, ServerConfig};
use std::{
    sync::Arc,
    net::{SocketAddr},
};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};


pub(crate) fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> anyhow::Result<(Endpoint, CertificateDer<'static>)> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
}

fn configure_server()
-> anyhow::Result<(ServerConfig, CertificateDer<'static>)> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}
