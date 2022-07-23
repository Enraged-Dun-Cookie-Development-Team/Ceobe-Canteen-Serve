use axum::{
    routing::{get, post},
    Router,
};

pub struct BakeryMansionBackend;

pub(super) fn bakery_mansion_router() -> Router {
    Router::new()
        .route("/upload", post(BakeryMansionBackend::save_mansion))
        .route("/getInfo", get(BakeryMansionBackend::get_mansion))
        .route("/getId", get(BakeryMansionBackend::get_recent_id))
        .route("/delete", post(BakeryMansionBackend::remove_mansion))
}
