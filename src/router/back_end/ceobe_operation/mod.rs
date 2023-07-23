pub use announcement::CeobeOperationAnnouncement;
use axum::Router;
pub use resource::CeobeOpResource;
pub use version::CeobeOpVersion;
pub use video::CeobeOperationVideo;

use self::{
    announcement::announcement_router, resource::resource_router,
    version::version_router, video::video_router,
};
use crate::{
    middleware::authorize::AuthorizeLayer,
    new_auth_level,
    utils::user_authorize::auth_level::prefabs::{Chef, Cooker},
};

mod announcement;
mod resource;
mod version;
mod video;

pub(super) fn ceobe_operation_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/announcement", announcement_router())
        .nest("/video", video_router())
        .nest("/version", version_router())
        .nest("/resource", resource_router())
        .route_layer(AuthorizeLayer::<CeobeOperationAuth>::new())
}

new_auth_level! {
    pub CeobeOperationAuth=>[
        Chef
        Cooker
    ]
}
