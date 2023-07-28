pub use analyze::AnalyzeCookieInside;
use axum::Router;

use self::analyze::analyze_router;
use super::ServerRoute;

mod analyze;

pub(super) fn inside_router() -> ServerRoute {
    Router::new().nest("/analyze", analyze_router())
}
