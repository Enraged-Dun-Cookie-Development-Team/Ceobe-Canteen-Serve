use axum::{routing::{post, delete, get}, Router};

use crate::router::ServerRoute;

use super::FetcherConfigControllers;

pub fn fetcher_global_config() -> ServerRoute {
    Router::new() 
    .route(
        "/uploadGlobalConfig",
        post(FetcherConfigControllers::upload_global_configs),
    )
    .route(
        "/getGlobalConfig",
        get(FetcherConfigControllers::get_global_configs),
    )
}