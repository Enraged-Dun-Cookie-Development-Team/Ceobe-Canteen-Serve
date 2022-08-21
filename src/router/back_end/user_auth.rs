use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer,
    utils::user_authorize::auth_level::prefabs::Chef,
};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> Router {
    Router::new()
        .route("/create", post(UserAuthBackend::create_user))
        .route("/info", get(UserAuthBackend::get_info))
        .route("/changeUsername", post(UserAuthBackend::change_username))
        .route("/changePassword", post(UserAuthBackend::change_password))
        .route_layer(AuthorizeLayer::<Chef>::new())
        // no middle ware cover
        .route("/login", post(UserAuthBackend::login))
}
