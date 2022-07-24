use axum::{routing::get, Router};
use tower::limit::ConcurrencyLimitLayer;

pub struct CeoboOperationVideo;

pub(super) fn video_router() -> Router {
    Router::new()
        .route("/detail", get(CeoboOperationVideo::get_video_detail))
        .layer(ConcurrencyLimitLayer::new(5))
}
