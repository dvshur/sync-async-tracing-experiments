use std::convert::Infallible;

use crate::repo;
use tracing::trace_span;
use warp::Filter;

pub async fn start(repo: repo::Sync) {
    let with_repo = warp::any().map(move || repo.clone());

    let request_tracing = warp::trace(|info| {
        trace_span!(
            "request",
            method = %info.method(),
            path = %info.path(),
        )
    });

    let go = warp::path::path("go")
        .and(warp::path::end())
        .and(with_repo.clone())
        .and_then(|r: repo::Sync| async move {
            tokio::task::spawn_blocking(move || r.fetch(())).await;
            Result::<_, Infallible>::Ok(warp::reply())
        })
        .with(request_tracing);

    warp::serve(go).run(([0, 0, 0, 0], 8080)).await
}
