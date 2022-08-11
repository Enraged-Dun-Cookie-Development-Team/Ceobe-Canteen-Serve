use axum::Router;

use self::{video::video_router, announcement::announcement_router, version::version_router};

mod video;
mod announcement;
mod version;
pub use video::CeobeOperationVideoFrontend;
pub use announcement::CeobeOperationAnnouncementFrontend;
pub use version::CeobeOperationVersionFrontend;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/version", version_router())
}
