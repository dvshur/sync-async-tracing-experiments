mod repo;
mod web;

use opentelemetry::global;
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    // enable tracing
    {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("blocking")
            .with_agent_endpoint("0.0.0.0:6831")
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap();

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let fmt_layer = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish();

        fmt_layer.with(opentelemetry).try_init().unwrap();
    }

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
