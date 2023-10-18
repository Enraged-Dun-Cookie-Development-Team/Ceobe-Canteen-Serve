use axum::{routing::get, Router};

pub struct CeobeOperationToolLinkFrontend;

pub(super) fn tool_link_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/list", get(CeobeOperationToolLinkFrontend::list))
}
