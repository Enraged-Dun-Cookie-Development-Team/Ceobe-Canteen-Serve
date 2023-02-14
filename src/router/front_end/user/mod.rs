use axum::{routing::{post, get}, Router};

use crate::router::ServerRoute;

pub struct CeobeUserFrontend;

pub(super) fn ceobe_user_router() -> ServerRoute {
    Router::new()
        .route(
            "/createUser",
            post(CeobeUserFrontend::register),
        )
        .route(
            "/datasourceConfig",
            get(CeobeUserFrontend::get_datasource_config_by_user),
        )
        .route(
            "/updateDatasourceConfig",
            post(CeobeUserFrontend::update_datasource_config_by_user),
        )
}