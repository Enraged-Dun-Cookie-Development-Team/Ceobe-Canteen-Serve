use axum::Router;

use self::{datasource::datasource_router, fetcher::fetcher_router};
use crate::router::ServerRoute;
pub mod datasource;
pub mod fetcher;
pub use self::datasource::ConfigDatasourceFrontend;
pub use self::fetcher::ConfigFetcherFrontend;

pub(super) fn config_router() -> ServerRoute {
    Router::new()
        .nest("/datasource", datasource_router())
        .nest("/fetcher", fetcher_router())
}
