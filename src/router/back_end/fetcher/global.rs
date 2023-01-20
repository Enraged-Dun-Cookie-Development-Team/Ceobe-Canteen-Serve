use axum::{
    routing::{get, post},
    Router,
};

use super::FetcherConfigControllers;
use crate::router::ServerRoute;

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
