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
fn router_v1() -> impl PrepareRouteEffect<State> {
    Nest::new("/api/v1", router::root_route())
}

/// 配置Fallback
#[prepare(RouterFallback)]
fn router_fallback<S>() -> impl PrepareRouteEffect<S>
where
    S: Send + Sync + 'static + Clone,
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
