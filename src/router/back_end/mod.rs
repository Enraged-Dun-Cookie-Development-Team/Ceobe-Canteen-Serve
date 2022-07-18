mod bakery_mansion;
mod user_auth;
use actix_web::Scope;
pub use bakery_mansion::BakeryMansionBackend;
pub use user_auth::UserAuthBackend;

use self::{
    bakery_mansion::bakery_mansion_router, user_auth::user_auth_router,
};

pub(super) fn back_end_router() -> Scope {
    actix_web::web::scope("/admin")
        .service(bakery_mansion_router())
        .service(user_auth_router())
}
