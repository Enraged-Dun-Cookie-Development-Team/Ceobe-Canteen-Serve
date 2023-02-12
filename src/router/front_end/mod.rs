mod user;
mod bakery_mansion;
mod ceobe_operation;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;
pub use user::CeobeUserFrontend;
pub use ceobe_operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationResourceFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend
};

use self::{
    bakery_mansion::bakery_mansion_router,
    ceobe_operation::ceobe_operation_router,
    user::ceobe_user_router,
};
use super::ServerRoute;
pub(super) fn front_end_router() -> ServerRoute {
    Router::new()
        .nest("/bakery", bakery_mansion_router())
        .nest("/operate", ceobe_operation_router())
        .nest("/user", ceobe_user_router())
}
