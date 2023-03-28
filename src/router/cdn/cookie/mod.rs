use self::temp::temp_router;
use crate::router::ServerRoute;
mod temp;
use axum::Router;
pub use temp::CdnCookieTempFrontend;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new().nest("/temp", temp_router())
}
