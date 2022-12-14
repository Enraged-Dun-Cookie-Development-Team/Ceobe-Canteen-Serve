use axum::{routing::get, Router};

pub struct CeobeOperationResourceFrontend;

pub(super) fn resource_router() -> crate::router::ServerRoute
{
    Router::new()
        .route("/get", get(CeobeOperationResourceFrontend::resource_list))
}
