use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::{instrument, trace};

#[derive(Debug, Clone)]
pub struct Sync {
    concurrent_threads: Arc<AtomicUsize>,
}

impl Sync {
    pub fn new() -> Self {
        Self {
            concurrent_threads: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[instrument]
    pub fn fetch<A: Debug>(&self, a: A) -> A {
        let threads_before = self.concurrent_threads.fetch_add(1, Ordering::SeqCst);
        let threads = threads_before + 1;

        trace!(threads, "start");

        std::thread::sleep(std::time::Duration::from_secs(2));

        self.concurrent_threads.fetch_sub(1, Ordering::SeqCst);

        trace!(threads, "end");

        a
    }
}

// #[derive(Debug, Clone)]
// pub struct Async {}

// impl Async {
//     pub fn new() -> Self {
//         Self {}
//     }

//     #[instrument]
//     pub async fn fetch<A: Debug>(&self, a: A) -> A {
//         a
//     }
// }
