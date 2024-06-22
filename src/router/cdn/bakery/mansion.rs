use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct CdnBakeryMansionFrontend;

pub(super) fn mansion_router() -> ServerRoute {
    Router::new()
        .route(
            "/mansionInfo",
            get(CdnBakeryMansionFrontend::get_mansion_with_time),
        )
        .route("/mansionId", get(CdnBakeryMansionFrontend::get_all_id))
        .route(
            "/mansion/recentPredict",
            get(CdnBakeryMansionFrontend::recent_mansion_predict),
        )
}
