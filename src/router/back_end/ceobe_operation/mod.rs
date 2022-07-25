use axum::Router;

use self::{announcement::announcement_router, video::video_router};

mod announcement;
mod video;

pub use announcement::CeobeOperationAnnouncement;
pub use video::CeobeOperationVideo;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new()
        .nest("/announcement", announcement_router())
        .nest("/video", video_router())
}
