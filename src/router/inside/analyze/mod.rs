use axum::Router;
pub use cookie::AnalyzeCookieInside;

use self::cookie::cookie_router;
use super::ServerRoute;

mod cookie;

pub(super) fn analyze_router() -> ServerRoute {
    Router::new().nest("/cookie", cookie_router())
}
