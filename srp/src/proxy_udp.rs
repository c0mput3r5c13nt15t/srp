use quinn::Connection;
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;

pub async fn proxy_udp_stream(
    _connection: Connection,
    _endpoint_addr: SocketAddr,
    _shutdown: CancellationToken,
) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("udp client not yet implemented"))
}
