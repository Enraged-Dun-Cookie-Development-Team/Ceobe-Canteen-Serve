use axum::Router;
pub use cookie::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
pub use operate::CdnOperateToolLinkFrontend;
use crate::router::cdn::operate::operate_router;

use self::cookie::cookie_router;
use super::ServerRoute;

mod cookie;
mod operate;

pub(super) fn cdn_router() -> ServerRoute {
    Router::new().nest("/cookie", cookie_router())
        .nest("/operate", operate_router())
}
