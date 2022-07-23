mod bakery_mansion;
mod user_auth;
use axum::Router;
pub use bakery_mansion::BakeryMansionBackend;
pub use user_auth::UserAuthBackend;

use self::{
    bakery_mansion::bakery_mansion_router, user_auth::user_auth_router,
};

pub(super) fn back_end_router() -> Router {
    Router::new()
        .nest("/user", user_auth_router())
        .nest("/mansion", bakery_mansion_router())
}
