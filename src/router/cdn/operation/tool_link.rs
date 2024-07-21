use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct CdnOperateToolLinkFrontend;

pub(super) fn tool_list_router() -> ServerRoute {
    Router::new().route("/list", get(CdnOperateToolLinkFrontend::list))
}
