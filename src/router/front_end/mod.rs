mod bakery_mansion;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;

use self::bakery_mansion::bakery_mansion_router;
pub(super) fn front_end_router() -> Router {
    Router::new().nest("/bakery", bakery_mansion_router())
}
