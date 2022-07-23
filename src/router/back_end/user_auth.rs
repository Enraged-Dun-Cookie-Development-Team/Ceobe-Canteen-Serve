use axum::{
    routing::{get, post},
    Router,
};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> Router {
    // web::scope("/user")
    Router::new()
        .route("/create", post(UserAuthBackend::create_user))
        .route("/login", post(UserAuthBackend::login))
        .route("/info", get(UserAuthBackend::get_info))
        .route("/changeUsername", post(UserAuthBackend::change_username))
        .route("/changePassword", post(UserAuthBackend::change_password))
}
