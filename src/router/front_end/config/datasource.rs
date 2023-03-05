use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct ConfigDatasourceFrontend;

pub(super) fn datasource_router() -> ServerRoute {
    Router::new()
        .route("/list", get(ConfigDatasourceFrontend::datasource_list))
}
