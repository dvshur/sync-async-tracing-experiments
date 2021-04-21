mod api;
mod repo;

use futures::future::join_all;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub async fn start() {
    let mut handlers = vec![];

    let repo = repo::Sync::new();

    for i in 0..1000usize {
        let repo = repo.clone();

        handlers.push(tokio::task::spawn(async move { repo.fetch(i) }));
        // handlers.push(tokio::task::spawn_blocking(move || repo.fetch(i)));
    }

    join_all(handlers).await;

    println!("All threads finished.");
}
