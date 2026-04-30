use log::{error, info};
use quinn::Connection;
use std::net::SocketAddr;
use tokio::{io::AsyncWriteExt, net::TcpStream, task::JoinSet};
use tokio_util::sync::CancellationToken;

async fn handle_tcp_stream(
    mut send: quinn::SendStream,
    mut recv: quinn::RecvStream,
    endpoint_addr: SocketAddr,
) {
    let result = async {
        let tcp = TcpStream::connect(endpoint_addr).await?;
        let (mut tcp_r, mut tcp_w) = tcp.into_split();

        let quic_to_tcp = async {
            tokio::io::copy(&mut recv, &mut tcp_w).await?;
            tcp_w.shutdown().await
        };

        let tcp_to_quic = async {
            tokio::io::copy(&mut tcp_r, &mut send).await?;
            send.finish()?;
            Ok::<_, std::io::Error>(())
        };

        tokio::try_join!(quic_to_tcp, tcp_to_quic)?;

        Ok::<(), anyhow::Error>(())
    }
    .await;

    if let Err(e) = result {
        error!("[client stream] error: {e}");
    }
}

pub async fn proxy_tcp_stream(
    connection: Connection,
    endpoint_addr: SocketAddr,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                info!("shutdown received, closing QUIC connection");
                connection.close(0u32.into(), b"client shutdown".as_ref());
                break;
            }

            accept = connection.accept_bi() => {
                let (send, recv) = match accept {
                    Ok(v) => v,
                    Err(e) => {
                        error!("accept_bi error: {e}");
                        break;
                    }
                };

                let addr = endpoint_addr;

                tasks.spawn(async move {
                    handle_tcp_stream(send, recv, addr).await;
                });
            }
        }
    }

    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            error!("task join error: {e}");
        }
    }

    Ok(())
}
