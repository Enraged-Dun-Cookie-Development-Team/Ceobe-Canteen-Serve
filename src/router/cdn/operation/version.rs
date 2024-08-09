use axum::{routing::get, Router};

pub struct CdnOperationVersion;

pub(super) fn version_router() -> crate::router::ServerRoute {
    Router::new().route("/fetch", get(CdnOperationVersion::release_version))
}
