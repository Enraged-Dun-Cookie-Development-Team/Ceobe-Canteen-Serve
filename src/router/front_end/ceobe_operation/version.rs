use axum::{routing::get, Router};

pub struct CeobeOperationVersionFrontend;

pub(super) fn version_router() -> Router {
    Router::new()
        .route("/app", get(CeobeOperationVersionFrontend::app_version))
        // .route("/plugin", get(CeobeOperationVersionFrontend::plugin_version))
}
