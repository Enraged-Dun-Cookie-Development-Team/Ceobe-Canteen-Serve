use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct CookieInfoFrontend;

pub(super) fn info_router() -> ServerRoute {
    Router::new()
        .route("/count", get(CookieInfoFrontend::cookie_count))
}
