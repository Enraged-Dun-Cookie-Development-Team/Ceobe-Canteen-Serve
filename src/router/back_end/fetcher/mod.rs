use axum::{routing::post, Router};

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
            "/uploadGlobalConfig",
            post(FetcherConfigControllers::upload_global_config),
        )
        .route_layer(AuthorizeLayer::<Chef>::new())
}
