use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct CookieSearchFrontend;

pub(super) fn search_router() -> ServerRoute {
    Router::new().route("/list", get(CookieSearchFrontend::search_list))
}
