mod cookie;


use axum::Router;

use self::cookie::cookie_router;
pub use cookie::AnalyzeCookieInside;

use super::ServerRoute;
pub(super) fn analyze_router() -> ServerRoute {
    Router::new()
        .nest("/cookie", cookie_router())
}
