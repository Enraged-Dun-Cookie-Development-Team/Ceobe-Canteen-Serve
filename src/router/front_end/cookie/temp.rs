

use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct CookieTempFrontend;

pub(super) fn temp_router() -> ServerRoute {
    Router::new()
        .route("/cookieList", get(CookieTempFrontend::cookie_list))
}
