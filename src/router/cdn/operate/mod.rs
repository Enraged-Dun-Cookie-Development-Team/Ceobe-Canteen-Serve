use axum::Router;
use crate::router::ServerRoute;

pub use tool_link::CdnOperateToolLinkFrontend;

mod tool_link;

pub(super) fn operate_router() -> ServerRoute {
    Router::new()
        .nest("/toolLink", crate::router::cdn::operate::tool_link::tool_list_router())
}
