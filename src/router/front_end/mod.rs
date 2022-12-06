mod bakery_mansion;
mod ceobe_operation;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;
pub use ceobe_operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationResourceFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend,
};

use self::{
    bakery_mansion::bakery_mansion_router,
    ceobe_operation::ceobe_operation_router,
};
pub(super) fn front_end_router<S:Clone + Send +Sync>() -> Router<S>{
    Router::new()
        .nest("/bakery", bakery_mansion_router())
        .nest("/operate", ceobe_operation_router())
}
