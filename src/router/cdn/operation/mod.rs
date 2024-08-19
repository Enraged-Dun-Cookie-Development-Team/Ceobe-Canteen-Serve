pub use announcement::CdnOperationAnnouncementFrontend;
use axum::Router;
use release_version_service::ReleaseVersionController;
pub use resource::CdnOperationResourceFrontend;
use serve_utils::{endpoint::CDN, ControllerRouterExt};
pub use video::CdnOperationVideoFrontend;

use self::{
    announcement::announcement_router, resource::resource_router,
    video::video_router,
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod video;

pub(super) fn operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/resource", resource_router())
        .nest_controller(ReleaseVersionController, CDN)
}
