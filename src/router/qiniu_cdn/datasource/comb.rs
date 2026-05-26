use axum::{Router, routing::get};

use crate::router::ServerRoute;

pub struct QiniuCdnDatasourceCombFrontend;

pub(super) fn comb_router() -> ServerRoute {
    Router::new().route(
        "/:comb_id",
        get(QiniuCdnDatasourceCombFrontend::get_newest_cookie),
    )
}
