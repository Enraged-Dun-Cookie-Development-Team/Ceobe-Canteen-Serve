use axum::{routing::get, Router};

pub struct CeobeOperationVideoFrontend;

pub(super) fn video_router<S:Clone + Send +Sync>() -> Router<S> {
    Router::new().route("/list", get(CeobeOperationVideoFrontend::list_all))
}
