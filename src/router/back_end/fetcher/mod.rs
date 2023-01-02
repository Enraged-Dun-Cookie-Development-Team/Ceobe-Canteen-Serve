mod global;
mod detail;
pub mod datasource;
mod platform;
use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer, router::ServerRoute,
    utils::user_authorize::auth_level::prefabs::Chef,
};

use self::{platform::fetcher_platform_config, datasource::fetcher_datasource_config, global::fetcher_global_config, detail::fetcher_detail_config};
pub struct FetcherConfigControllers;

pub fn fetcher_config() -> ServerRoute {
    Router::new()
        .route(
            "/uploadAvatar",
            post(FetcherConfigControllers::upload_avatar),
        )
        .merge(fetcher_platform_config())
        .merge(fetcher_datasource_config())
        .merge(fetcher_global_config())
        .merge(fetcher_detail_config())
        .route_layer(AuthorizeLayer::<Chef>::new())
}
