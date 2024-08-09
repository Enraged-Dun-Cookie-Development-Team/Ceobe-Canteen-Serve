use axum::Router;
use axum::routing::get;

pub struct CdnOperationVersion;

pub(super) fn version_router()->crate::router::ServerRoute{
    Router::new()
        .route("/fetch",get(CdnOperationVersion::release_version))
}