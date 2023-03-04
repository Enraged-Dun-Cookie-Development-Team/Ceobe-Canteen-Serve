use axum::{
    routing::{get, post},
    Router,
};

use crate::{router::ServerRoute, middleware::mob::MobVerifyLayer};

pub struct CeobeUserFrontend;

pub(super) fn ceobe_user_router() -> ServerRoute {
    Router::new()
        .route(
            "/datasourceConfig",
            get(CeobeUserFrontend::get_datasource_config_by_user),
        )
        .route(
            "/updateDatasourceConfig",
            post(CeobeUserFrontend::update_datasource_config_by_user),
        )
        .route(
            "/refreshTime",
            post(CeobeUserFrontend::update_user_access_time),
        )
        .route_layer(MobVerifyLayer::new())
        .route("/createUser", post(CeobeUserFrontend::register))
}
