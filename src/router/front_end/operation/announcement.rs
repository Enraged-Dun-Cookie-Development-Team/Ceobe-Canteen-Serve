use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct CeobeOperationAnnouncementFrontend;

pub(super) fn announcement_router() -> ServerRoute {
    Router::new().route(
        "/list",
        get(CeobeOperationAnnouncementFrontend::get_announcement_list),
    )
}
