use std::time::{Duration};
use tokio::time;


pub struct healthcheck_client {
    http: reqwest::Client,
    base_url: String,
    uuid: String,
}

impl healthcheck_client {
    pub fn new(base_url: &str, uuid: &str) -> Self{
        Self {
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            base_url: base_url.trim_end_matches('/').to_string(),
            uuid: uuid.to_string(),
        }
    }

    pub async fn success(&self) -> anyhow::Result<()>{
        self.http
            .get(format!("{}/{}", self.base_url, self.uuid))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn start(&self) -> anyhow::Result<()>{
        self.http
            .get(format!("{}/{}/start", self.base_url, self.uuid))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn stop(&self) -> anyhow::Result<()>{
        self.http
            .get(format!("{}/{}/stop", self.base_url, self.uuid))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn fail(&self) -> anyhow::Result<()>{
        self.http
            .get(format!("{}/{}/fail", self.base_url, self.uuid))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn start_period<F, Fut>(
        self,
        interval_secs: u64,
        is_healthy: F,
    ) where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = bool> + Send,
    {
        let mut interval = time::interval(Duration::from_secs(interval_secs));
        loop{
            interval.tick().await;
            if is_healthy().await {
                if let Err(e) = self.success().await{
                    eprintln!("healthcheck failed: {e}");
                }
            }
            else {
                if let Err(e) = self.fail().await {
                    eprintln!("healthcheck fail-ping was sent: {e}");
                }
            }
        }
    }
}