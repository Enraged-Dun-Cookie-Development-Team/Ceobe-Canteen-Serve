use axum::{
    Router,
    routing::{get, post},
};

pub struct CeobeOperationAnnouncement;

pub(super) fn announcement_router() -> crate::router::ServerRoute {
    Router::new()
        .route(
            "/get",
            get(CeobeOperationAnnouncement::get_announcement_list),
        )
        .route(
            "/submitList",
            post(CeobeOperationAnnouncement::update_announcement_list),
        )
}
