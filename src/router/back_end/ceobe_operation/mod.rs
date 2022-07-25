use axum::Router;

use self::announcement::announcement_router;

mod announcement;

pub use announcement::CeobeOperationAnnouncement;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new().nest("/announcement", announcement_router())
}
