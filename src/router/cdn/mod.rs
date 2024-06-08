use axum::Router;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
use operation::operation_router;
pub use operation::{
    CdnOperationAnnouncementFrontend, CdnOperationResourceFrontend,
    CdnOperationVideoFrontend,
};

use self::cookie::cookie_router;
use super::ServerRoute;

mod cookie;
mod operation;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new()
        .nest("/cookie", cookie_router())
        .nest("/operate", operation_router())
}
