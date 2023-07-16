use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct BakeryMansionFrontend;

pub(super) fn mansion_router() -> ServerRoute {
    Router::new()
        .route(
            "/mansionInfo",
            get(BakeryMansionFrontend::get_mansion_with_time),
        )
        .route("/mansionId", get(BakeryMansionFrontend::get_all_id))
        .route(
            "/mansion/recentPredict",
            get(BakeryMansionFrontend::recent_mansion_predict),
        )
}
