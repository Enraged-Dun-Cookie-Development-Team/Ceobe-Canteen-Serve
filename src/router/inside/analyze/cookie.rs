use axum::Router;

use crate::router::ServerRoute;

pub struct AnalyzeCookieInside;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new()
}