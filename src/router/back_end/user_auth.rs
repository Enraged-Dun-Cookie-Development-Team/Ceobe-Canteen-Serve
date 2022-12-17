use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer,
    utils::user_authorize::auth_level::prefabs::{Any, Chef},
};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/create", post(UserAuthBackend::create_user))
        .route("/userList", get(UserAuthBackend::user_list))
        .route("/changeAuth", post(UserAuthBackend::change_auth))
        .route("/deleteUser", delete(UserAuthBackend::delete_one_user))
        .route_layer(AuthorizeLayer::<Chef>::new())
        .merge(
            Router::new()
                .route(
                    "/changeUsername",
                    post(UserAuthBackend::change_username),
                )
                .route(
                    "/changePassword",
                    post(UserAuthBackend::change_password),
                )
                .route("/info", get(UserAuthBackend::get_info))
                .route_layer(AuthorizeLayer::<Any>::new()),
        )
        // no middle ware cover
        .route("/login", post(UserAuthBackend::login))
}
