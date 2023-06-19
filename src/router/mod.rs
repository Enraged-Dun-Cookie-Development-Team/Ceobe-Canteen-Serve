mod back_end;
mod cdn;
mod front_end;
mod inside;

use axum::{routing::get, Router};
pub use back_end::{
    BakeryMansionBackend, CeobeOpResource, CeobeOpVersion,
    CeobeOperationAnnouncement, CeobeOperationVideo,
    FetcherConfigControllers, UserAuthBackend,
};
pub use cdn::{CdnCookieMainListFrontend, CdnCookieTempFrontend};
pub use front_end::{
    BakeryMansionFrontend, CeobeOperationAnnouncementFrontend,
    CeobeOperationResourceFrontend, CeobeOperationVersionFrontend,
    CeobeOperationVideoFrontend, CeobeUserFrontend, ConfigDatasourceFrontend,
    CookieTerraComicFrontend, CookieInfoFrontend
};
pub use inside::AnalyzeCookieInside;

pub type ServerRoute = Router<State>;

use self::{
    back_end::back_end_router, cdn::cdn_router, front_end::front_end_router,
    inside::inside_router,
};
use crate::bootstrap::State;

pub fn root_route() -> ServerRoute {
    Router::new()
        .nest("/canteen", front_end_router())
        .nest("/admin", back_end_router())
        .nest("/cdn", cdn_router())
        .nest("/inside", inside_router())
        .route(
            "/panic",
            get(|| async {
                #[cfg(debug_assertions)]
                {
                    panic!("测试 Panic");
                }
                #[cfg(not(debug_assertions))]
                resp_result::RespResult::<_, crate::error::NotAnError>::ok(
                    "不可以Panic",
                )
            }),
        )
}
