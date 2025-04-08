use axum::{routing::post, Router};
use authorize_server::admin::AdminAuthorizeLayer;
use authorize_server::admin::base_roles::Chef;
use self::{
    datasource::fetcher_datasource_config, detail::fetcher_detail_config,
    global::fetcher_global_config, platform::fetcher_platform_config,
};
use crate::{router::ServerRoute,

};

pub mod datasource;
mod detail;
mod global;
mod platform;

pub struct FetcherConfigControllers;

pub fn fetcher_config() -> ServerRoute {
    Router::new()
        .route(
            "/uploadDatasourceAvatar",
            post(FetcherConfigControllers::upload_avatar),
        )
        .merge(fetcher_platform_config())
        .merge(fetcher_datasource_config())
        .merge(fetcher_global_config())
        .merge(fetcher_detail_config())
        .route_layer(AdminAuthorizeLayer::<Chef>::new())
}
