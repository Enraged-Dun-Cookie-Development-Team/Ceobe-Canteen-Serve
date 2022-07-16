mod bakery_mansion;
use actix_web::{web, Scope};
pub use bakery_mansion::BakeryMansionFrontend;

use self::bakery_mansion::bakery_mansion_router;
pub(super) fn front_end_router() -> Scope {
    web::scope("/canteen").service(bakery_mansion_router())
}
