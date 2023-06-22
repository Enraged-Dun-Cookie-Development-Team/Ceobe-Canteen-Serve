use axum::Router;
mod mansion;
use crate::router::ServerRoute;
pub use mansion::BakeryMansionFrontend;

use self::mansion::mansion_router;

pub(super) fn bakery_router() -> ServerRoute {
    Router::new().merge(mansion_router())
}
