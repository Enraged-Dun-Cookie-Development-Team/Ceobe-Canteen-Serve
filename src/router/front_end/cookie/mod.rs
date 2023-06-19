use self::{terra_comic::terra_comic_router, info::info_router};
use crate::router::ServerRoute;

mod info;
mod terra_comic;
use axum::Router;
pub use terra_comic::CookieTerraComicFrontend;
pub use info::CookieInfoFrontend;

pub(super) fn ceobe_cookie_router() -> ServerRoute {
    Router::new().nest("/terraComic", terra_comic_router())
        .nest("/info", info_router())
}
