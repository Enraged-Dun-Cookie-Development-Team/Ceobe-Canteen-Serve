use axum::Router;
use bakery::bakery_router;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
use operation::operation_router;
pub use operation::{
    CdnOperationAnnouncementFrontend, CdnOperationResourceFrontend,
    CdnOperationVideoFrontend,
};
pub use bakery::CdnBakeryMansionFrontend;

use self::cookie::cookie_router;
use super::ServerRoute;

mod cookie;
mod operation;
mod bakery;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new()
        .nest("/cookie", cookie_router())
        .nest("/operate", operation_router())
        .nest("/bakery", bakery_router())
}
