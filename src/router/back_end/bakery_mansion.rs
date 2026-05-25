use authorize_server::mix_role_gen;
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    middleware::authorize::AuthorizeLayer,
    utils::user_authorize::auth_level::prefabs::{Architect, Chef},
};

pub struct BakeryMansionBackend;

pub(super) fn bakery_mansion_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/upload", post(BakeryMansionBackend::save_mansion))
        .route("/getInfo", get(BakeryMansionBackend::get_mansion))
        .route("/getId", get(BakeryMansionBackend::get_recent_id))
        .route("/delete", post(BakeryMansionBackend::remove_mansion))
        .route_layer(AuthorizeLayer::<MansionAuth>::new())
}

mix_role_gen! {
    pub MansionAuth=>[
        Chef
        Architect
    ]
}
