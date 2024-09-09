use axum::{routing::get, Router};
pub use back_end::{
    BakeryMansionBackend, CeobeCookieNewestBackend, CeobeOpResource,
    CeobeOpToolLink, CeobeOpVersion, CeobeOperationAnnouncement,
    CeobeOperationVideo, FetcherConfigControllers, UserAuthBackend,
};
pub use cdn::{
    CdnBakeryMansionFrontend, CdnCookieMainListFrontend,
    CdnCookieTempFrontend, CdnOperateToolLinkFrontend,
    CdnOperationAnnouncementFrontend, CdnOperationResourceFrontend,
    CdnOperationVideoFrontend,
};
pub use front_end::{
    BakeryMansionFrontend, CeobeOperationAnnouncementFrontend,
    CeobeOperationResourceFrontend, CeobeOperationToolLinkFrontend,
    CeobeOperationVersionFrontend, CeobeOperationVideoFrontend,
    CeobeUserFrontend, ConfigDatasourceFrontend, ConfigFetcherFrontend,
    CookieInfoFrontend, CookieSearchFrontend, CookieTerraComicFrontend,
};
pub use inside::AnalyzeCookieInside;
pub use qiniu_cdn::QiniuCdnDatasourceCombFrontend;

use self::{
    back_end::back_end_router, cdn::cdn_router, front_end::front_end_router,
    inside::inside_router, qiniu_cdn::qiniu_cdn_router,
};
use crate::bootstrap::State;

mod back_end;
mod cdn;
mod front_end;
mod inside;
mod qiniu_cdn;

pub type ServerRoute = Router<State>;

pub fn root_route() -> ServerRoute {
    Router::new()
        .nest("/canteen", front_end_router())
        .nest("/admin", back_end_router())
        .nest("/cdn", cdn_router())
        .nest("/inside", inside_router())
        .nest("/qiniuCdn", qiniu_cdn_router())
        .route(
            "/panic",
            get(|| {
                async {
                    #[cfg(debug_assertions)]
                    {
                        let f =||->&str{panic!("测试 Panic")};
                        f()
                    }
                    #[cfg(not(debug_assertions))]
                    axum_resp_result::RespResult::<_, crate::error::NotAnError>::ok(
                        "不可以Panic",
                    )
                }
            }),
        )
}
