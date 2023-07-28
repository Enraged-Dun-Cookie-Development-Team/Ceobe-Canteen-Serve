use axum::Router;

use self::{datasource::datasource_router, fetcher::fetcher_router};
pub use self::{
    datasource::ConfigDatasourceFrontend, fetcher::ConfigFetcherFrontend,
};
use crate::router::ServerRoute;

pub mod datasource;
pub mod fetcher;

pub(super) fn config_router() -> ServerRoute {
    Router::new()
        .nest("/datasource", datasource_router())
        .nest("/fetcher", fetcher_router())
}
