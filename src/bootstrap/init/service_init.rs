use axum::handler::Handler;
use axum_starter::{
    graceful::SetGraceful,
    prepare,
    router::{Fallback, Nest},
    PreparedEffect,
};
use futures::FutureExt;
use tracing::log;

use crate::{error::not_exist, router};

/// 配置router
#[prepare(RouteV1)]
fn router_v1() -> impl PreparedEffect {
    Nest::new("/api/v1", router::root_route())
}

/// 配置Fallback
#[prepare(RouterFallback)]
fn router_fallback() -> impl PreparedEffect {
    Fallback::new(not_exist.into_service())
}

pub async fn graceful_shutdown() -> impl PreparedEffect {
    SetGraceful::new(tokio::signal::ctrl_c().map(|_| {
        log::info!("收到退出信号");
    }))
}
