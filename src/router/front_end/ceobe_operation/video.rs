use axum::{routing::get, Router};

pub struct CeobeOperationVideoFrontend;

pub(super) fn video_router() -> Router {
    // Router::new().route("/list", get(CeobeOperationVideoFrontend::list_all))
}
