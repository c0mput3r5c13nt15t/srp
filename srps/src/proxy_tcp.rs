use log::{error, info};
use quinn::Connection;
use tokio::{net::TcpListener, task::JoinSet};
use tokio_util::sync::CancellationToken;

pub async fn proxy_tcp_stream(
    connection: Connection,
    listener: TcpListener,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();

    loop {
        tokio::select! {
            // 1. Explicit shutdown (Ctrl+C)
            _ = shutdown.cancelled() => {
                info!("[stream] shutdown received (token)");
                connection.close(0u32.into(), b"shutdown".as_ref());
                break;
            }

            // 2. QUIC connection closed by peer
            _ = connection.closed() => {
                info!("[stream] shutdown received (connection closed)");
                break;
            }

            // 3. Accept TCP connections
            accept = listener.accept() => {
                let (tcp_stream, _) = match accept {
                    Ok(v) => v,
                    Err(e) => {
                        error!("[stream] accept error: {e}");
                        continue;
                    }
                };

                let conn = connection.clone();

                tasks.spawn(async move {
                    let result = async {
                        let (mut send, mut recv) = conn.open_bi().await?;

                        let (mut r, mut w) = tcp_stream.into_split();

                        let up = tokio::io::copy(&mut r, &mut send);
                        let down = tokio::io::copy(&mut recv, &mut w);

                        tokio::try_join!(up, down)?;

                        send.finish()?;
                        Ok::<(), anyhow::Error>(())
                    }
                    .await;

                    if let Err(e) = result {
                        error!("[stream tasks] error: {e:?}");
                    }
                });
            }
        }
    }

    // 4. Drain all active stream tasks
    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            error!("[stream] task join error: {e:?}");
        }
    }

    Ok(())
}
