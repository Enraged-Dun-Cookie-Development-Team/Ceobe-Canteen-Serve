mod back_end;
mod front_end;

use axum::{routing::get, Router};
pub use back_end::{
    BakeryMansionBackend, CeobeOpResource, CeobeOpVersion,
    CeobeOperationAnnouncement, CeobeOperationVideo, UserAuthBackend,
};
pub use front_end::BakeryMansionFrontend;

use self::{back_end::back_end_router, front_end::front_end_router};

pub fn root_route() -> Router {
    Router::new()
        .nest("/canteen", front_end_router())
        .nest("/admin", back_end_router())
        .route(
            "/panic",
            get(|| {
                async {
                    #[cfg(debug_assertions)]
                    {
                        panic!("测试 Panic");
                    }
                    #[cfg(not(debug_assertions))]
                    resp_result::RespResult::<_, crate::error::NotAnError>::ok("不可以Panic")
                }
            }),
        )
}
