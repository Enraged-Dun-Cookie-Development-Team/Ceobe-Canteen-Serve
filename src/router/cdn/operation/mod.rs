pub use announcement::CdnOperationAnnouncementFrontend;
use release_version_service::ReleaseVersionController;
pub use resource::CdnOperationResourceFrontend;
use serve_utils::{endpoint::CDN, ControllerRouter};
pub use tool_link::CdnOperateToolLinkFrontend;
pub use video::CdnOperationVideoFrontend;

use self::{
    announcement::announcement_router, resource::resource_router,
    tool_link::tool_list_router, video::video_router,
};
use crate::router::ServerRoute;

mod announcement;
mod resource;
mod tool_link;
mod video;

pub(super) fn operation_router() -> ServerRoute {
    ControllerRouter::new(CDN)
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/resource", resource_router())
        .nest_controller(ReleaseVersionController)
        .nest("/toolLink", tool_list_router())
        .into()
}
