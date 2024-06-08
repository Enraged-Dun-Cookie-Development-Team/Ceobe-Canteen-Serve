use axum::Router;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
pub use operation::{CdnOperationAnnouncementFrontend, CdnOperationVideoFrontend, CdnOperationResourceFrontend};
use operation::operation_router;

use self::cookie::cookie_router;
use super::ServerRoute;

mod cookie;
mod operation;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new().nest("/cookie", cookie_router())
    .nest("/operate", operation_router())
}
