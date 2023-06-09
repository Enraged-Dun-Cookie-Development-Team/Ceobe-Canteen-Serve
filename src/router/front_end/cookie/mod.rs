mod search;
use self::{terra_comic::terra_comic_router, search::search_router};
use crate::router::ServerRoute;

mod terra_comic;
use axum::Router;
pub use terra_comic::CookieTerraComicFrontend;
pub use search::CookieSearchFrontend;

pub(super) fn ceobe_cookie_router() -> ServerRoute {
    Router::new().nest("/terraComic", terra_comic_router())
                .nest("/search", search_router())
}
