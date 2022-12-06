use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer,
    utils::user_authorize::auth_level::prefabs::{Architect, Chef},
};

pub struct BakeryMansionBackend;

pub(super) fn bakery_mansion_router<S:Clone + Send +Sync>() -> Router<S> {
    Router::new()
        .route("/upload", post(BakeryMansionBackend::save_mansion))
        .route("/getInfo", get(BakeryMansionBackend::get_mansion))
        .route("/getId", get(BakeryMansionBackend::get_recent_id))
        .route("/delete", post(BakeryMansionBackend::remove_mansion))
        .route_layer(AuthorizeLayer::<MansionAuth>::new())
}

crate::new_auth_level! {
    pub MansionAuth=>[
        Chef
        Architect
    ]
}
