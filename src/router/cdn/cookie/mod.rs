use axum::Router;
pub use main_list::CdnCookieMainListFrontend;
pub use temp::CdnCookieTempFrontend;

use self::{main_list::main_list_router, temp::temp_router};
use crate::router::ServerRoute;

mod main_list;
mod temp;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new()
        .nest("/temp", temp_router())
        .nest("/mainList", main_list_router())
}
