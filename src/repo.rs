use std::fmt::Debug;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct Sync {}

impl Sync {
    pub fn new() -> Self {
        Self {}
    }

    #[instrument]
    pub fn fetch<A: Debug>(&self, a: A) -> A {
        a
    }
}

#[derive(Debug, Clone)]
pub struct Async {}

impl Async {
    pub fn new() -> Self {
        Self {}
    }

    #[instrument]
    pub async fn fetch<A: Debug>(&self, a: A) -> A {
        a
    }
}
