use actix_web::{Scope, web};


pub struct BakeryMansionFrontend;

pub(super) fn bakery_mansion_router()->Scope{
    web::scope("/bakery")
    .service(web::resource("/mansionInfo").route(
        web::get().to(BakeryMansionFrontend::get_mansion_with_time),
    ))
    .service(
        web::resource("/mansionId")
            .route(web::get().to(BakeryMansionFrontend::get_all_id)),
    )

}