mod main_list;
use self::{temp::temp_router, main_list::main_list_router};
use crate::router::ServerRoute;
mod temp;
use axum::Router;
pub use temp::{CdnCookieTempFrontend};
pub use main_list::CdnCookieMainListFrontend;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new()
        .nest("/temp", temp_router())
        .nest("/mainList", main_list_router())
}
