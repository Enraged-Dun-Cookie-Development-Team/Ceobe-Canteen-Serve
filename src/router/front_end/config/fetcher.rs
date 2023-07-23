use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct ConfigFetcherFrontend;

pub fn fetcher_router() -> ServerRoute {
    Router::new().route(
        "/standaloneConfig/:combine_id",
        get(ConfigFetcherFrontend::standalone_fetcher_config),
    )
}
