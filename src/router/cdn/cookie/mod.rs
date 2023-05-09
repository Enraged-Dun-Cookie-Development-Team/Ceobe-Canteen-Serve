mod main_list;
use self::{main_list::main_list_router, temp::temp_router};
use crate::router::ServerRoute;
mod temp;
use axum::Router;
pub use main_list::CdnCookieMainListFrontend;
pub use temp::CdnCookieTempFrontend;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new()
        .nest("/temp", temp_router())
        .nest("/mainList", main_list_router())
}
