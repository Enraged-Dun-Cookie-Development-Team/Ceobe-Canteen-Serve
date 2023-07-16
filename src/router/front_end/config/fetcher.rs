use axum::{routing::get, Router};

pub struct ConfigFetcherFrontend;

use crate::router::ServerRoute;

pub fn fetcher_router() -> ServerRoute {
    Router::new().route(
        "/standaloneConfig/:combine_id",
        get(ConfigFetcherFrontend::standalone_fetcher_config),
    )
}
