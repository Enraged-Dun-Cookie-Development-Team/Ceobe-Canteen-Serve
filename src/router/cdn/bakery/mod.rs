use axum::Router;
pub use mansion::CdnBakeryMansionFrontend;

use self::mansion::mansion_router;
use crate::router::ServerRoute;

mod mansion;

pub(super) fn bakery_router() -> ServerRoute {
    Router::new().merge(mansion_router())
}
