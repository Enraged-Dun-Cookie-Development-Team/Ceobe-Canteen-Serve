mod back_end;
mod front_end;
use axum::Router;
pub use back_end::{BakeryMansionBackend, UserAuthBackend};
pub use front_end::BakeryMansionFrontend;

use self::{back_end::back_end_router, front_end::front_end_router};

pub fn root_route() -> Router {
    Router::new()
        .nest("/canteen", front_end_router())
        .nest("/admin", back_end_router())
}
