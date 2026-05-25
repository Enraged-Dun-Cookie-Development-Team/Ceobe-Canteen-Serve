use axum::{
    Router,
    routing::{get, post},
};

pub struct CeobeOpResource;

pub fn resource_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/submitList", post(CeobeOpResource::upload_resource))
        .route("/list", get(CeobeOpResource::get_resource))
}
