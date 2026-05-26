use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct CdnCookieTempFrontend;

pub(super) fn temp_router() -> ServerRoute {
    Router::new()
        .route("/cookieList", get(CdnCookieTempFrontend::cookie_list))
}
