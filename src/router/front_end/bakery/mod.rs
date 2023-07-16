use axum::Router;
mod mansion;
pub use mansion::BakeryMansionFrontend;

use self::mansion::mansion_router;
use crate::router::ServerRoute;

pub(super) fn bakery_router() -> ServerRoute {
    Router::new().merge(mansion_router())
}
