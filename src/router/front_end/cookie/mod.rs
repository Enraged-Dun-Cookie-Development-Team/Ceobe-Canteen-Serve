mod search;
use self::{
    info::info_router, search::search_router, terra_comic::terra_comic_router,
};
use crate::router::ServerRoute;

mod info;
mod terra_comic;
use axum::Router;
pub use info::CookieInfoFrontend;
pub use search::CookieSearchFrontend;
pub use terra_comic::CookieTerraComicFrontend;

pub(super) fn ceobe_cookie_router() -> ServerRoute {
    Router::new()
        .nest("/terraComic", terra_comic_router())
        .nest("/search", search_router())
        .nest("/info", info_router())
}
