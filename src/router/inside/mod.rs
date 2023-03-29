mod analyze;

use axum::Router;

use self::analyze::analyze_router;

pub use analyze::AnalyzeCookieInside;

use super::ServerRoute;
pub(super) fn inside_router() -> ServerRoute {
    Router::new()
        .nest("/analyze", analyze_router())
}
