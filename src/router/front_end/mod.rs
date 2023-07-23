use axum::Router;
pub use config::{ConfigDatasourceFrontend, ConfigFetcherFrontend};
pub use cookie::{
    CookieInfoFrontend, CookieSearchFrontend, CookieTerraComicFrontend,
};
pub use operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationResourceFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend,
};
pub use user::CeobeUserFrontend;

pub use self::bakery::BakeryMansionFrontend;
use self::{
    bakery::bakery_router, config::config_router,
    cookie::ceobe_cookie_router, operation::ceobe_operation_router,
    user::ceobe_user_router,
};
use super::ServerRoute;

mod bakery;
mod config;
mod cookie;
mod operation;
mod user;

pub(super) fn front_end_router() -> ServerRoute {
    Router::new()
        .nest("/bakery", bakery_router())
        .nest("/operate", ceobe_operation_router())
        .nest("/user", ceobe_user_router())
        .nest("/config", config_router())
        .nest("/cookie", ceobe_cookie_router())
}
