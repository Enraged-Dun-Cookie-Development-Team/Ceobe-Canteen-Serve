pub use announcement::CdnOperationAnnouncementFrontend;
use axum::Router;
pub use resource::CdnOperationResourceFrontend;
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
    Router::new()
        .nest("/video", video_router())
        .nest("/announcement", announcement_router())
        .nest("/resource", resource_router())
        .nest("/toolLink", tool_list_router())
}
