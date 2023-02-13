use axum::Router;

use crate::router::ServerRoute;

use self::datasource::datasource_router;
pub mod datasource;
pub use datasource::ConfigDatasourceFrontend;

pub(super) fn config_router() -> ServerRoute {
    Router::new()
        .nest("/datasource", datasource_router())
}