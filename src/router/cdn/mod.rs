use axum::Router;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};

use self::cookie::cookie_router;
use super::ServerRoute;
mod cookie;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new().nest("/cookie", cookie_router())
}
