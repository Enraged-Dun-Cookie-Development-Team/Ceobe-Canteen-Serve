use axum::{routing::get, Router};

pub struct CdnOperationResourceFrontend;

pub(super) fn resource_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/get", get(CdnOperationResourceFrontend::resource_list))
}
