use axum::{routing::post, Router};

use crate::{router::ServerRoute, middleware::datasource_comb_mutex::DatasourceCombMutexLayer};

pub struct AnalyzeCookieInside;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new()
        .route("/new", post(AnalyzeCookieInside::new_cookie).route_layer(DatasourceCombMutexLayer))
}
