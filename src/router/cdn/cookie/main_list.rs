use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct CdnCookieMainListFrontend;

pub(super) fn main_list_router() -> ServerRoute {
    Router::new()
        .route("/cookieList", get(CdnCookieMainListFrontend::cookie_list))
}
