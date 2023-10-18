
pub use announcement::CeobeOperationAnnouncementFrontend;
use axum::Router;
pub use resource::CeobeOperationResourceFrontend;
pub use version::CeobeOperationVersionFrontend;
pub use video::CeobeOperationVideoFrontend;
pub use tool_link::CeobeOperationToolLinkFrontend;

use self::{
    announcement::announcement_router, resource::resource_router,
    version::version_router, video::video_router, tool_link::tool_link_router
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod version;
mod video;
mod tool_link;

pub(super) fn ceobe_operation_router() -> ServerRoute {
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/version", version_router())
        .nest("/resource", resource_router())
        .nest("/toolLink", tool_link_router())
}
