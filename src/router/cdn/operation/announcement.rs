use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct CdnOperationAnnouncementFrontend;

pub(super) fn announcement_router() -> ServerRoute {
    Router::new().route(
        "/list",
        get(CdnOperationAnnouncementFrontend::get_announcement_list),
    )
}
