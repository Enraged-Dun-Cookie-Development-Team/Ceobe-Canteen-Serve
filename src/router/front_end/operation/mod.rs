pub use announcement::CeobeOperationAnnouncementFrontend;
use axum::Router;
pub use resource::CeobeOperationResourceFrontend;
pub use version::CeobeOperationVersionFrontend;
pub use video::CeobeOperationVideoFrontend;

use self::{
    announcement::announcement_router, resource::resource_router,
    version::version_router, video::video_router,
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod version;
mod video;

pub(super) fn ceobe_operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/version", version_router())
        .nest("/resource", resource_router())
}
