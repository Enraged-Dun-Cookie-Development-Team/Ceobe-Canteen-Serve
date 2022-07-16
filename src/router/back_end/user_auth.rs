use actix_web::{web, Scope};

pub struct UserAuthBackend;

pub(super) fn user_auth_router() -> Scope {
    web::scope("/user")
        .service(
            web::resource("/create")
                .route(web::post().to(UserAuthBackend::create_user)),
        )
        .service(
            web::resource("/login")
                .route(web::post().to(UserAuthBackend::login)),
        )
        .service(
            web::resource("/info")
                .route(web::get().to(UserAuthBackend::get_info)),
        )
        .service(
            web::resource("/changeUsername")
                .route(web::post().to(UserAuthBackend::change_username)),
        )
        .service(
            web::resource("/changePassword")
                .route(web::post().to(UserAuthBackend::change_password)),
        )
}
