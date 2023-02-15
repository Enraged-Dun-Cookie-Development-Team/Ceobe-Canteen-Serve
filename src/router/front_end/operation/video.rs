use axum::{routing::get, Router};

pub struct CeobeOperationVideoFrontend;

pub(super) fn video_router() -> crate::router::ServerRoute {
    Router::new().route("/list", get(CeobeOperationVideoFrontend::list_all))
}
