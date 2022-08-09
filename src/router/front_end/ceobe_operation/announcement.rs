use axum::{routing::get, Router};

pub struct CeobeOperationAnnouncementFrontend;

pub(super) fn announcement_router() -> Router {
    // Router::new().route("/list", get(CeobeOperationAnnouncementFrontend::list_all()))
}
