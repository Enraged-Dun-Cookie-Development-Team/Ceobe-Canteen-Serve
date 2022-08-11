use axum::Router;

use self::{
    announcement::announcement_router, version::version_router,
    video::video_router,
};

mod announcement;
mod version;
mod video;
pub use announcement::CeobeOperationAnnouncementFrontend;
pub use version::CeobeOperationVersionFrontend;
pub use video::CeobeOperationVideoFrontend;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/version", version_router())
}