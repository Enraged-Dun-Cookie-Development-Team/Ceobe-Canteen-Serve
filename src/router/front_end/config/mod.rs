use axum::Router;

use self::datasource::datasource_router;
use crate::router::ServerRoute;
pub mod datasource;
pub use datasource::ConfigDatasourceFrontend;

pub(super) fn config_router() -> ServerRoute {
    Router::new().nest("/datasource", datasource_router())
}
