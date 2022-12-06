use axum::{routing::get, Router};

pub struct CeobeOperationResourceFrontend;

pub(super) fn resource_router<S:Clone + Send +Sync+ 'static>() -> Router<S> {
    Router::new()
        .route("/get", get(CeobeOperationResourceFrontend::resource_list))
}
