use axum::{
    routing::{delete, get, post},
    Router,
};

use super::FetcherConfigControllers;
use crate::router::ServerRoute;

pub fn fetcher_datasource_config() -> ServerRoute {
    Router::new()
        .route(
            "/getPlatformAndDatasourceOption",
            get(FetcherConfigControllers::get_platform_and_datasource_list),
        )
        .route(
            "/getDatasourceList",
            get(FetcherConfigControllers::get_datasource_list),
        )
        .route(
            "/createDatasource",
            post(FetcherConfigControllers::create_datasource_config),
        )
        .route(
            "/updateDatasource",
            post(FetcherConfigControllers::update_datasource_config),
        )
        .route(
            "/deleteDatasource",
            delete(FetcherConfigControllers::delete_datasource_config),
        )
}
