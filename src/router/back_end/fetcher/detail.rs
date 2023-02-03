use axum::{
    routing::{get, post},
    Router,
};

use super::FetcherConfigControllers;
use crate::router::ServerRoute;

pub fn fetcher_detail_config() -> ServerRoute {
    Router::new()
    .route(
        "/allPlatformList",
        get(FetcherConfigControllers::get_platform_all_list_with_basic_info),
    )
    .route(
        "/getAllDatasourceList",
        get(FetcherConfigControllers::get_datasource_name_list),
    )
    .route(
        "/getFetcherLiveNumber",
        get(FetcherConfigControllers::get_fetcher_max_live_number),
    )
    .route(
        "/uploadFetcherConfig",
        post(FetcherConfigControllers::upload_fetchers_configs),
    )
    .route(
        "/getFetcherConfigList",
        get(FetcherConfigControllers::get_fetchers_configs),
    )
}
