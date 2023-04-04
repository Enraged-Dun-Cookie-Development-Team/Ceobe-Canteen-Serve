use axum::{routing::post, Router};

use crate::{
    middleware::datasource_comb_mutex::DatasourceCombMutexLayer,
    router::ServerRoute,
};

pub struct AnalyzeCookieInside;

pub(super) fn cookie_router() -> ServerRoute {
    Router::new().route(
        "/new",
        post(AnalyzeCookieInside::new_cookie)
            .route_layer(DatasourceCombMutexLayer),
    )
}
