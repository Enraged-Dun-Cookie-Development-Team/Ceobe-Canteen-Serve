use axum::Router;
pub use comb::QiniuCdnDatasourceCombFrontend;

use self::comb::comb_router;
use crate::router::ServerRoute;

mod comb;

pub(super) fn datasource_router() -> ServerRoute {
    Router::new()
        .nest("/datasource-comb", comb_router())
}
