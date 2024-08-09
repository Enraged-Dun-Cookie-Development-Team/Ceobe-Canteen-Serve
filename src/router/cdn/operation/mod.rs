pub use announcement::CdnOperationAnnouncementFrontend;
use axum::Router;
pub use resource::CdnOperationResourceFrontend;
pub use version::CdnOperationVersion;
pub use video::CdnOperationVideoFrontend;

use self::{
    announcement::announcement_router, resource::resource_router,
    version::version_router, video::video_router,
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod version;
mod video;

pub(super) fn operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/resource", resource_router())
        .nest("/version", version_router())
}
