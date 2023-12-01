use axum::{
    routing::{delete, get, post},
    Router,
};

pub struct CeobeOpToolLink;

pub fn tool_link_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/create", post(CeobeOpToolLink::create_one))
        .route("/update", post(CeobeOpToolLink::update_one))
        .route("/delete", delete(CeobeOpToolLink::delete_one))
        .route("/list", get(CeobeOpToolLink::list))
        .route("/uploadAvatar", post(CeobeOpToolLink::upload_avatar))
}
