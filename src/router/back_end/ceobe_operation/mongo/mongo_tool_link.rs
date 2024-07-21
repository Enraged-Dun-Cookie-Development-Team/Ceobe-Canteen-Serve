use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn mongo_tool_link_router() -> crate::router::ServerRoute {
    Router::new()
        .route(
            "/list",
            get(crate::router::CdnOperateToolLinkFrontend::list),
        )
        .route(
            "/create",
            post(crate::router::CdnOperateToolLinkFrontend::create_one),
        )
        .route(
            "/update",
            post(crate::router::CdnOperateToolLinkFrontend::update_one),
        )
        .route(
            "/delete",
            delete(crate::router::CdnOperateToolLinkFrontend::delete_one),
        )
}
