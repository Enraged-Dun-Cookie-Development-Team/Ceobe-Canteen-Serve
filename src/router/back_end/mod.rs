mod bakery_mansion;
use actix_web::Scope;
pub use bakery_mansion::BakeryMansionBackend;

use self::bakery_mansion::bakery_mansion_router;

pub(super) fn back_end_router() -> Scope {
    actix_web::web::scope("/admin").service(bakery_mansion_router())
}
