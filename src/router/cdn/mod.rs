use axum::Router;

use self::cookie::cookie_router;

use super::ServerRoute;

pub use cookie::CdnCookieTempFrontend;
mod cookie;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new()
        .nest("/cookie", cookie_router())
}