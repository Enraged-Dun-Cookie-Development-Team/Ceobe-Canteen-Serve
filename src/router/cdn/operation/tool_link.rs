use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::router::ServerRoute;

pub struct CdnOperateToolLinkFrontend;

pub(super) fn tool_list_router() -> ServerRoute {
    Router::new()
        .route("/list", get(CdnOperateToolLinkFrontend::list))
        .route("/create", post(CdnOperateToolLinkFrontend::create_one))
        .route("/update", post(CdnOperateToolLinkFrontend::update))
        .route("/delete", delete(CdnOperateToolLinkFrontend::delete))
}
