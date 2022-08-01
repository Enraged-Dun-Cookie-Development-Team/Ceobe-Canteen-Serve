mod version;
use axum::Router;

use self::{
    announcement::announcement_router, version::version_router,
    video::video_router,
};

mod announcement;
mod video;

pub use announcement::CeobeOperationAnnouncement;
pub use version::CeobeOpVersion;
pub use video::CeobeOperationVideo;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new()
        .nest("/announcement", announcement_router())
        .nest("/video", video_router())
        .nest("/version", version_router())
}
