use self::terra_comic::terra_comic_router;
use crate::router::ServerRoute;

mod terra_comic;
use axum::Router;
pub use terra_comic::CookieTerraComicFrontend;

pub(super) fn ceobe_cookie_router() -> ServerRoute {
    Router::new().nest("/terraComic", terra_comic_router())
}
