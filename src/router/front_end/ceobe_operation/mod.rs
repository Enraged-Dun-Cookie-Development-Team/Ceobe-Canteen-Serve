mod resource;
use axum::Router;

use crate::router::ServerRoute;

use self::{
    announcement::announcement_router, resource::resource_router,
    version::version_router, video::video_router,
};

mod announcement;
mod version;
mod video;
pub use announcement::CeobeOperationAnnouncementFrontend;
pub use resource::CeobeOperationResourceFrontend;
pub use version::CeobeOperationVersionFrontend;
pub use video::CeobeOperationVideoFrontend;

pub(super) fn ceobe_operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/version", version_router())
        .nest("/resource", resource_router())
}
