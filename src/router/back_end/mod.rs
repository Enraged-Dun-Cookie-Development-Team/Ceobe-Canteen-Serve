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
pub use fetcher::FetcherConfigControllers;
pub use user_auth::UserAuthBackend;

use self::{
    bakery_mansion::bakery_mansion_router,
    ceobe_operation::ceobe_operation_router, user_auth::user_auth_router,
};

pub(super) fn back_end_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/fetcherConfig", fetcher::fetcher_config())
        .nest("/user", user_auth_router())
        .nest("/mansion", bakery_mansion_router())
        .merge(ceobe_operation_router())
}
