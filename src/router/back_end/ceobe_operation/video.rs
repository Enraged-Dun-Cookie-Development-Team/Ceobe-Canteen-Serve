use axum::{
    routing::{get, post},
    Router,
};
use tower::limit::ConcurrencyLimitLayer;

use crate::{
    middleware::authorize::AuthorizeLayer,
    new_auth_level,
    utils::user_authorize::auth_level::prefabs::{Chef, Cooker},
};

pub struct CeobeOperationVideo;

pub(super) fn video_router() -> Router {
    Router::new()
        .route(
            "/detail",
            get(CeobeOperationVideo::get_video_detail)
                .layer(ConcurrencyLimitLayer::new(5)),
        )
        .route("/list", get(CeobeOperationVideo::list_all))
        .route("/submitList", post(CeobeOperationVideo::update_list))
        .route_layer(AuthorizeLayer::<VideoAuth>::new())
}

new_auth_level! {
    pub VideoAuth => [
        Chef
        Cooker
    ]
}
