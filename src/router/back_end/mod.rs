use axum::Router;
pub use bakery_mansion::BakeryMansionBackend;
pub use ceobe_cookie::CeobeCookieNewestBackend;
pub use ceobe_operation::{
    CeobeOpResource, CeobeOpVersion, CeobeOperationAnnouncement,
    CeobeOperationVideo,
};
pub use user_auth::UserAuthBackend;

pub use self::fetcher::FetcherConfigControllers;
use self::{
    bakery_mansion::bakery_mansion_router, ceobe_cookie::ceobe_cookie_router,
    ceobe_operation::ceobe_operation_router, fetcher::fetcher_config,
    user_auth::user_auth_router,
};

mod bakery_mansion;
pub mod ceobe_operation;
pub mod fetcher;

mod ceobe_cookie;
mod user_auth;

pub(super) fn back_end_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/fetcherConfig", fetcher_config())
        .nest("/user", user_auth_router())
        .nest("/mansion", bakery_mansion_router())
        .nest("/cookie", ceobe_cookie_router())
        .merge(ceobe_operation_router())
}
