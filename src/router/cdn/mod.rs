use axum::Router;
use bakery::bakery_router;
pub use bakery::CdnBakeryMansionFrontend;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
use operation::operation_router;
pub use operation::{
    CdnOperateToolLinkFrontend, CdnOperationAnnouncementFrontend,
    CdnOperationResourceFrontend, CdnOperationVideoFrontend,
};

use self::cookie::cookie_router;
use super::ServerRoute;

mod bakery;
mod cookie;
mod operation;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new()
        .nest("/cookie", cookie_router())
        .nest("/operate", operation_router())
        .nest("/bakery", bakery_router())
}
