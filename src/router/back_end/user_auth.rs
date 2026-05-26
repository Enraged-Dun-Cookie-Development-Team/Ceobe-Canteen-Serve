use authorize_server::admin::{
    AdminAuthorizeLayer,
    base_roles::{Any, Chef},
};
use axum::{
    Router,
    routing::{delete, get, post},
};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/create", post(UserAuthBackend::create_user))
        .route("/userList", get(UserAuthBackend::user_list))
        .route("/changeAuth", post(UserAuthBackend::change_auth))
        .route("/deleteUser", delete(UserAuthBackend::delete_one_user))
        .route_layer(AdminAuthorizeLayer::<Chef>::new())
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
                .route_layer(AdminAuthorizeLayer::<Any>::new()),
        )
        // no middle ware cover
        .route("/login", post(UserAuthBackend::login))
}
