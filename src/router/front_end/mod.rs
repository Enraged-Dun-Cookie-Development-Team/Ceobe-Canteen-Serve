mod bakery_mansion;
mod ceobe_operation;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;
pub use ceobe_operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationVersionFrontend,
    CeobeOperationVideoFrontend, CeobeOperationResourceFrontend
};

use self::{
    bakery_mansion::bakery_mansion_router,
    ceobe_operation::ceobe_operation_router,
};
pub(super) fn front_end_router() -> Router {
    Router::new()
        .nest("/bakery", bakery_mansion_router())
        .nest("/operate", ceobe_operation_router())
}
