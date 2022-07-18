mod back_end;
mod front_end;
use actix_web::{web, Scope};
pub use back_end::{BakeryMansionBackend, UserAuthBackend};
pub use front_end::BakeryMansionFrontend;

use self::{back_end::back_end_router, front_end::front_end_router};

pub fn root_route() -> Scope {
    web::scope("/api/v1")
        .service(front_end_router())
        .service(back_end_router())
}
