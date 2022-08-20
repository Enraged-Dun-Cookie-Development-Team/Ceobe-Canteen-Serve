use axum::{routing::post, Router};

pub struct CeobeOpVersion;

pub fn version_router() -> Router {
    Router::new()
        .route("/plugin", post(CeobeOpVersion::update_plugin))
        .route("/phone", post(CeobeOpVersion::create_app_version))
}
