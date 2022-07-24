use axum::Router;

use self::video::video_router;

mod video;

pub use video::CeoboOperationVideo;

pub(super) fn ceobo_operation_router() -> Router {
    Router::new().nest("/video", video_router())
}
