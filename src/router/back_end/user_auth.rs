use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer,
    utils::user_authorize::auth_level::prefabs::{Any, Chef},
};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> Router {
    Router::new()
        .merge(
            Router::new()
                .route("/create", post(UserAuthBackend::create_user))
                .route(
                    "/changeUsername",
                    post(UserAuthBackend::change_username),
                )
                .route(
                    "/changePassword",
                    post(UserAuthBackend::change_password),
                )
                .route_layer(AuthorizeLayer::<Chef>::new()),
        )
        .merge(
            Router::new()
                .route("/info", get(UserAuthBackend::get_info))
                .route_layer(AuthorizeLayer::<Any>::new()),
        )
        // no middle ware cover
        .route("/login", post(UserAuthBackend::login))
}
