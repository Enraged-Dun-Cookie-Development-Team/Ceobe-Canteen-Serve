use axum::{routing::{get, delete}, routing::post, Router};

use crate::{
    middleware::authorize::AuthorizeLayer, router::ServerRoute,
    utils::user_authorize::auth_level::prefabs::Chef,
};
pub struct FetcherConfigControllers;

pub fn fetcher_config() -> ServerRoute {
    Router::new()
        .route(
            "/uploadAvatar",
            post(FetcherConfigControllers::upload_avatar),
        )
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
        .route(
            "/getPlatformAndDatasourceOption",
            get(FetcherConfigControllers::get_platform_and_datasource_list),
        )
        .route(
            "/uploadGlobalConfig",
            post(FetcherConfigControllers::upload_global_configs),
        )
        .route(
            "/getGlobalConfig",
            get(FetcherConfigControllers::get_global_configs),
        )
        .route_layer(AuthorizeLayer::<Chef>::new())
}
