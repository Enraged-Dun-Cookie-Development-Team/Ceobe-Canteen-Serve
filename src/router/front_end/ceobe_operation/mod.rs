use axum::Router;

use self::video::video_router;

mod video;
pub use video::CeobeOperationVideoFrontend;

pub(super) fn ceobe_operation_router() -> Router {
    Router::new().nest("/video", video_router())
}
