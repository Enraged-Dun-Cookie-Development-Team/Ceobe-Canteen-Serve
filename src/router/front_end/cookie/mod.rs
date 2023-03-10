use self::temp::temp_router;
use crate::router::ServerRoute;
pub mod temp;
use axum::Router;
pub use temp::CookieTempFrontend;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new().nest("/temp", temp_router())
}
