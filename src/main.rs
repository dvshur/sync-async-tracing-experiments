mod api;
mod repo;

use futures::future::join_all;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[tokio::main]
async fn main() {
    let mut handlers = vec![];

    let heavy = Heavy::new();

    for i in 0..1000usize {
        // let i = i.clone();

        let heavy = heavy.clone();

        // handlers.push(tokio::task::spawn(async move { heavy.go(i) }));
        handlers.push(tokio::task::spawn_blocking(move || heavy.go(i)));
    }

    join_all(handlers).await;

    println!(
        "All threads finished.",
        // concurrent_threads.load(Ordering::SeqCst)
    );
}

#[derive(Clone)]
struct Heavy {
    concurrent_threads: Arc<AtomicUsize>,
    // max_concurrent_threads: Arc<AtomicUsize>,
}

impl Heavy {
    pub fn new() -> Self {
        Self {
            concurrent_threads: Arc::new(AtomicUsize::new(0)),
            // max_concurrent_threads: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn go(&self, thread_id: usize) {
        let threads_before = self.concurrent_threads.fetch_add(1, Ordering::SeqCst);

        // self.max_concurrent_threads.fetch_update(set_order, fetch_order, mutf)

        // if threads_before >= self.max_concurrent_threads.load() {
        //     // self.concurrent_threads.fetch_add(1, Ordering::SeqCst);
        //     self.max_concurrent_threads
        // }

        eprintln!("START | {} | {}", thread_id, threads_before + 1);

        std::thread::sleep(std::time::Duration::from_secs(2));

        let threads_before = self.concurrent_threads.fetch_sub(1, Ordering::SeqCst);
        eprintln!("END | {} | {}", thread_id, threads_before + 1);
    }
}
