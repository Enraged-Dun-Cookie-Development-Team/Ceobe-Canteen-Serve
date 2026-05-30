use axum::{Router, routing::get};

pub struct CdnOperationVideoFrontend;

pub(super) fn video_router() -> crate::router::ServerRoute {
    Router::new().route("/list", get(CdnOperationVideoFrontend::list_all))
}
