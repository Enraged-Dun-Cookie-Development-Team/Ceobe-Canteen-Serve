use axum::{routing::get, Router};

pub struct BakeryMansionFrontend;

pub(super) fn bakery_mansion_router<S:Clone + Send +Sync+ 'static>() -> Router<S> {
    Router::new()
        .route(
            "/mansionInfo",
            get(BakeryMansionFrontend::get_mansion_with_time),
        )
        .route("/mansionId", get(BakeryMansionFrontend::get_all_id))
}
