mod front_end;
mod back_end;
use actix_web::{web, Scope};

use self::{front_end::front_end_router, back_end::back_end_router};

pub use back_end::BakeryMansionBackend;
pub use front_end::BakeryMansionFrontend;
pub use back_end::UserAuthBackend;

pub fn root_route()->Scope{
    web::scope("/api/v1")
    .service(front_end_router())
    .service(back_end_router())
}

