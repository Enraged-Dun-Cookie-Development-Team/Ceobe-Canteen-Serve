use persistence::operate::FromRef;
use serve_utils::{
    axum::{
        routing::{get, post},
        Router,
    },
    endpoint::{AdminEnd, CDN},
    ControllerRoute, HandlerMapReject, HandlerResult,
};
use tencent_cloud_server::axum_starter::{
    PartTencentCloudManagerState, RequestClient,
};

mod admin_end;
mod cdn;

pub(crate) type Result<T> = HandlerResult<T, crate::ReleaseVersionController>;
pub(crate) type MapRejecter<T> =
    HandlerMapReject<T, crate::ReleaseVersionController>;

impl<S> ControllerRoute<S, CDN> for crate::ReleaseVersionController
where
    S: Send + Clone + Sync + 'static,
    PartTencentCloudManagerState: FromRef<S>,
    RequestClient: FromRef<S>,
{
    const BASE_URI: &'static str = "/version";

    fn route(self) -> Router<S> {
        Router::new()
            .route("/fetch", get(Self::fetch_version))
            .route("/all", get(Self::all_version_by_platform))
    }
}

impl<S> ControllerRoute<S, AdminEnd> for crate::ReleaseVersionController
where
    S: Send + Clone + Sync + 'static,
    PartTencentCloudManagerState: FromRef<S>,
    RequestClient: FromRef<S>,
{
    const BASE_URI: &'static str = "/releaseVersion";

    fn route(self) -> Router<S> {
        Router::new()
            .route("/markDelete", post(Self::mark_delete_version))
            .route("/create", post(Self::new_version))
            .route("/all", get(Self::all_version))
            .route("/modify", post(Self::modify_description))
    }
}
