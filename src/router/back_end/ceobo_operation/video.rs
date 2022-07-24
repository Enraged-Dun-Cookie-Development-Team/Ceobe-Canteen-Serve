use axum::{routing::{get, post}, Router};
use tower::limit::ConcurrencyLimitLayer;

pub struct CeoboOperationVideo;

pub(super) fn video_router() -> Router {
    Router::new()
        .route("/detail", get(CeoboOperationVideo::get_video_detail))
        .layer(ConcurrencyLimitLayer::new(5))
        .route("/list",get(CeoboOperationVideo::list_all))
        .route("/submitList",post(CeoboOperationVideo::update_list))
}
