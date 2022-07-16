use actix_web::{web, Scope};

pub struct BakeryMansionBackend;

pub(super) fn bakery_mansion_router() -> Scope {
    web::scope("/mansion")
        .service(
            web::resource("/upload")
                .route(web::post().to(BakeryMansionBackend::save_mansion)),
        )
        .service(
            web::resource("/getInfo")
                .route(web::get().to(BakeryMansionBackend::get_mansion)),
        )
        .service(
            web::resource("/getId")
                .route(web::get().to(BakeryMansionBackend::get_recent_id)),
        )
        .service(
            web::resource("/delete")
                .route(web::post().to(BakeryMansionBackend::remove_mansion)),
        )
}
