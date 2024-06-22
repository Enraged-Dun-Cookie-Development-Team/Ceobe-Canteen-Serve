use axum::{routing::{get, post}, Router};

use crate::router::ServerRoute;

pub struct CdnOperateToolLinkFrontend;

pub(super) fn tool_list_router() -> ServerRoute {
    Router::new()
        .route("/create", post(CdnOperateToolLinkFrontend::create_one))
        .route("/list", get(CdnOperateToolLinkFrontend::list))
}
