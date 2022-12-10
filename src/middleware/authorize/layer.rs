use axum::{body::Body, response::Response};
use http::Request;
use tower::{Layer, Service};
use tower_http::auth::{
    AsyncRequireAuthorization, AsyncRequireAuthorizationLayer,
};

use super::service::{self, AdminAuthorize};
use crate::utils::user_authorize::auth_level::AuthLevelVerify;

#[derive(Clone)]
pub struct AuthorizeLayer<L: AuthLevelVerify>(InnerLayer<L>);

impl<S, L> Layer<S> for AuthorizeLayer<L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    type Service = AsyncRequireAuthorization<S, AdminAuthorize<L>>;

    fn layer(&self, inner: S) -> Self::Service { self.0.layer(inner) }
}

impl<L: AuthLevelVerify> AuthorizeLayer<L> {
    pub fn new() -> Self { Self(InnerLayer::new(AdminAuthorize::default())) }
}

type InnerLayer<L> =
    AsyncRequireAuthorizationLayer<service::AdminAuthorize<L>>;
