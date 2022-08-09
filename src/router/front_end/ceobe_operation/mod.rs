mod announcement;
use axum::Router;

use self::{video::video_router, announcement::announcement_router};

mod video;
pub use video::CeobeOperationVideoFrontend;
pub use announcement::CeobeOperationAnnouncementFrontend;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
}
