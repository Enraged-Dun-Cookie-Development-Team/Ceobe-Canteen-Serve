use persistence::operate::FromRef;
use serve_utils::{
    axum::{
        routing::{get, post},
        Router,
    },
    endpoint::{AdminEnd, CDN},
    ControllerRouter, HandlerMapReject, HandlerResult,
};
use tencent_cloud_server::axum_starter::{
    PartTencentCloudManagerState, RequestClient,
};

mod admin_end;
mod cdn;

pub(crate) type Result<T> = HandlerResult<T, crate::ReleaseVersionController>;
pub(crate) type MapRejecter<T> =
    HandlerMapReject<T, crate::ReleaseVersionController>;

impl<S: Send + Clone + Sync + 'static> ControllerRouter<S, CDN>
    for crate::ReleaseVersionController
{
    const BASE_URI: &'static str = "/version";

    fn route(self) -> Router<S> {
        Router::new().route("/fetch", get(Self::release_version))
    }
}

impl<S> ControllerRouter<S, AdminEnd> for crate::ReleaseVersionController
where
    S: Send + Clone + Sync + 'static,
    PartTencentCloudManagerState: FromRef<S>,
    RequestClient: FromRef<S>,
{
    const BASE_URI: &'static str = "/release_version";

    fn route(self) -> Router<S> {
        Router::new()
            .route("/yank", post(Self::yank_version))
            .route("/create", post(Self::new_version))
            .route("/all", get(Self::all_version))
            .route("/total_num", get(Self::released_version_num))
    }
}
