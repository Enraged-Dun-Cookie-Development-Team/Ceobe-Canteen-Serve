use axum::{
    routing::{get, post},
    Router,
};

use crate::router::ServerRoute;

pub struct ConfigDatasourceFrontend;

pub(super) fn datasource_router() -> ServerRoute {
    Router::new()
        .route("/list", get(ConfigDatasourceFrontend::datasource_list))
        .route(
            "/standalone-fetcher-config/:combine_id",
            post(ConfigDatasourceFrontend::standalone_fetcher_config),
        )
}
