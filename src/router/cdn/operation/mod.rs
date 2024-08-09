pub use announcement::CdnOperationAnnouncementFrontend;
use axum::Router;
pub use resource::CdnOperationResourceFrontend;
pub use video::CdnOperationVideoFrontend;
pub use version::CdnOperationVersion;

use self::{
    announcement::announcement_router, resource::resource_router,
    video::video_router, version::version_router
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod video;
mod version;

pub(super) fn operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/resource", resource_router())
        .nest("/version",version_router())
}
