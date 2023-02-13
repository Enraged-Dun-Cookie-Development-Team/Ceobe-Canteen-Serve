use axum::{routing::post, Router};

use crate::router::ServerRoute;

pub struct CeobeUserFrontend;

pub(super) fn ceobe_user_router() -> ServerRoute {
    Router::new()
        .route(
            "/createUser",
            post(CeobeUserFrontend::register),
        )
}