use std::sync::Arc;
use std::time::Duration;
use tracing::{instrument, debug, info};

mod upstream_params {
    pub const REQUEST_TIMEOUT_SECS: u64 = 30;
    pub const CONNECT_TIMEOUT_SECS: u64 = 5;
}

#[instrument]
pub async fn start(timeout: Duration, workers: usize) {
    tokio::time::sleep(Duration::from_secs(5)).await;

    let client = Arc::new(
        reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(upstream_params::CONNECT_TIMEOUT_SECS))
            .timeout(Duration::from_secs(upstream_params::REQUEST_TIMEOUT_SECS))
            .pool_max_idle_per_host(workers + 4) // just in case
            .build()
            .unwrap(),
    );

    info!(timeout = timeout.as_secs(), workers, "shitstorm");

    let mut worker_handlers = vec![];

    for i in 0..workers {
        let client = client.clone();
        worker_handlers.push(tokio::spawn(async move {
            info!(id = i, "worker started");
            loop {
                debug!("requesting");
                client.get("http://0.0.0.0:8080/go").send().await.unwrap();
            }
        }))
    }

    let workers_handle = futures::future::join_all(worker_handlers);
    let timeout_handle = tokio::time::sleep(timeout);

    tokio::select! {
        _ = timeout_handle => {
            println!("operation timed out");
        }
        _ = workers_handle => {
            println!("all workers finished");
        }
    }

    // tokio::select!workers_handle, timeout_handle].await
}
