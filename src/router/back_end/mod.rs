mod bakery_mansion;
pub mod ceobe_operation;
pub mod fetcher;

mod user_auth;
use axum::Router;
pub use bakery_mansion::BakeryMansionBackend;
pub use ceobe_operation::{
    CeobeOpResource, CeobeOpVersion, CeobeOperationAnnouncement,
    CeobeOperationVideo,
};
pub use user_auth::UserAuthBackend;

pub use self::fetcher::FetcherConfigControllers;
use self::{
    bakery_mansion::bakery_mansion_router,
    ceobe_operation::ceobe_operation_router, fetcher::fetcher_config,
    user_auth::user_auth_router,
};

pub(super) fn back_end_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/fetcherConfig", fetcher_config())
        .nest("/user", user_auth_router())
        .nest("/mansion", bakery_mansion_router())
        .merge(ceobe_operation_router())
}
