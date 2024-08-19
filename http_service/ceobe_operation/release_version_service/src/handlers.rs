use serve_utils::{
    axum::{routing::get, Router},
    endpoint::CDN,
    ControllerRouter, HandlerMapReject, HandlerResult,
};
use serve_utils::axum::routing::post;
use serve_utils::endpoint::AdminEnd;

mod cdn;
mod admin_end;

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


impl<S: Send + Clone + Sync + 'static> ControllerRouter<S, AdminEnd>
for crate::ReleaseVersionController
{
    const BASE_URI: &'static str = "/version";

    fn route(self) -> Router<S> {
        Router::new().route("/yank", post(Self::yank_version))
    }
}

