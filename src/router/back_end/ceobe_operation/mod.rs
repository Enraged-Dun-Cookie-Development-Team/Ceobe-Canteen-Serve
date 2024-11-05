pub use announcement::CeobeOperationAnnouncement;
use release_version_service::ReleaseVersionController;
pub use resource::CeobeOpResource;
use serve_utils::{endpoint::AdminEnd, ControllerRouter};
pub use tool_link::CeobeOpToolLink;
pub use version::CeobeOpVersion;
pub use video::CeobeOperationVideo;

use self::{
    announcement::announcement_router, resource::resource_router,
    tool_link::tool_link_router, version::version_router,
    video::video_router,
};
use crate::{
    middleware::authorize::AuthorizeLayer,
    new_auth_level,
    utils::user_authorize::auth_level::prefabs::{Chef, Cooker, Porter},
};

mod announcement;
mod resource;
mod tool_link;
mod version;
mod video;

pub(super) fn ceobe_operation_router() -> crate::router::ServerRoute {
    ControllerRouter::new(AdminEnd)
        .nest("/announcement", announcement_router())
        .nest("/video", video_router())
        .nest("/version", version_router())
        .nest("/resource", resource_router())
        .nest_controller(ReleaseVersionController)
        .route_layer(AuthorizeLayer::<CeobeOperationAuth>::new())
        .nest("/toolLink", tool_link_router())
        .into()
}

new_auth_level! {
    pub CeobeOperationAuth=>[
        Chef
        Cooker
    ]
}
new_auth_level! {
    pub CeobeTools=>[
        Chef
        Cooker
        Porter
    ]
}
