use axum::body::Body;
use axum_starter::{
    prepare,
    router::{Fallback, Nest},
    PrepareRouteEffect,
};
use futures::FutureExt;
use tracing::info;

use crate::{bootstrap::State, error::not_exist, router};

/// 配置router
#[prepare(RouteV1)]
fn router_v1() -> impl PrepareRouteEffect<State, Body> {
    Nest::new("/api/v1", router::root_route())
}

/// 配置Fallback
#[prepare(RouterFallback)]
fn router_fallback<S, B>() -> impl PrepareRouteEffect<S, B>
where
    S: Send + Sync + 'static + Clone,
    B: Send + Sync + http_body::Body + 'static,
{
    Fallback::new(not_exist)
}

pub async fn graceful_shutdown() {
    tokio::signal::ctrl_c()
        .map(|_| {
            info!(signal.exit = true);
        })
        .await
}
