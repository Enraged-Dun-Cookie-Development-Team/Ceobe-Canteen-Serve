use axum::{
    routing::{get, post},
    Router,
};

pub struct CeobeOpResource;

pub fn resource_router<S:Clone + Send +Sync+ 'static>() -> Router<S> {
    Router::new()
        .route("/submitList", post(CeobeOpResource::upload_resource))
        .route("/list", get(CeobeOpResource::get_resource))
}
