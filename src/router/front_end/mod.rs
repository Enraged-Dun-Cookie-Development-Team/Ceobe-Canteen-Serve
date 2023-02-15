pub mod config;
mod user;
mod bakery_mansion;
mod operation;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;
pub use user::CeobeUserFrontend;
pub use operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationResourceFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend
};
pub use config::ConfigDatasourceFrontend;

use self::{
    bakery_mansion::bakery_mansion_router,
    operation::ceobe_operation_router,
    user::ceobe_user_router, config::config_router,
};
use super::ServerRoute;
pub(super) fn front_end_router() -> ServerRoute {
    Router::new()
        .nest("/bakery", bakery_mansion_router())
        .nest("/operate", ceobe_operation_router())
        .nest("/user", ceobe_user_router())
        .nest("/config", config_router())
}
