mod repo;
mod web;

use tracing::info;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    tracing_subscriber::fmt::init();

    let server_handler = web::server::start(repo::Sync::new());
    let client_handler = web::client::start(std::time::Duration::from_secs(10), 10);

    tokio::select! {
        _ = server_handler => {
            info!("server handler finished");
        }
        _ = client_handler => {
            info!("client handler finished");
        }
    }
}
