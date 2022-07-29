use axum::{
    routing::{get, post},
    Router,
};

use crate::{middleware::authorize::AuthorizeLayer, new_auth_level, utils::user_authorize::auth_level::prefabs::{Chef, Cooker}};

pub struct CeobeOperationAnnouncement;

pub(super) fn announcement_router() -> Router {
    Router::new()
        .route(
            "/get",
            get(CeobeOperationAnnouncement::get_announcement_list),
        )
        .route(
            "/submitList",
            post(CeobeOperationAnnouncement::update_announcement_list),
        )
        .route_layer(AuthorizeLayer::<AnnouncementAuth>::new())
}
new_auth_level! {
    pub AnnouncementAuth => [
        Chef
        Cooker
    ]
}