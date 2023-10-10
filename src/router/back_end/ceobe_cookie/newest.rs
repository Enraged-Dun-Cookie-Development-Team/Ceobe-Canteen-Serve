use axum::{routing::post, Router};

pub struct CeobeCookieNewestBackend;

pub(super) fn newest_router() -> crate::router::ServerRoute {
    Router::new().route(
        "/synchronousCombId",
        post(CeobeCookieNewestBackend::synchronous_qiniu_from_redis),
    )
}
