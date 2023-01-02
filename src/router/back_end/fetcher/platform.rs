use axum::{routing::{post, delete, get}, Router};

use crate::router::ServerRoute;

use super::FetcherConfigControllers;

pub fn fetcher_platform_config() -> ServerRoute {
    Router::new() 
    .route(
        "/platformList",
        get(FetcherConfigControllers::get_platform_list),
    )
    .route(
        "/createPlatform",
        post(FetcherConfigControllers::create_platform_config),
    )
    .route(
        "/updatePlatform",
        post(FetcherConfigControllers::update_platform_config),
    )
    .route(
        "/deletePlatform",
        delete(FetcherConfigControllers::delete_platform_config),
    )
}