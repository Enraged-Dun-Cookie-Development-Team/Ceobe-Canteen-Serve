mod bakery_mansion;
mod config;
mod operation;
mod user;
use axum::Router;
pub use bakery_mansion::BakeryMansionFrontend;
pub use config::ConfigDatasourceFrontend;
pub use operation::{
    CeobeOperationAnnouncementFrontend, CeobeOperationResourceFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend,
};
pub use user::CeobeUserFrontend;

use self::{
    bakery_mansion::bakery_mansion_router, config::config_router,
    operation::ceobe_operation_router, user::ceobe_user_router,
};
use super::ServerRoute;
pub(super) fn front_end_router() -> ServerRoute {
    Router::new()
        .nest("/bakery", bakery_mansion_router())
        .nest("/operate", ceobe_operation_router())
        .nest("/user", ceobe_user_router())
        .nest("/config", config_router())
}
