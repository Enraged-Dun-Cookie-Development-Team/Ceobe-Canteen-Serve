use axum::Router;
pub use datasource::QiniuCdnDatasourceCombFrontend;

use self::datasource::datasource_router;
use super::ServerRoute;

mod datasource;

pub(super) fn qiniu_cdn_router() -> ServerRoute {
    Router::new().merge(datasource_router())
}
