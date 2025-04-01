use axum::{body::Body, response::Response};
use http::Request;
use tower::{Layer, Service};
use tower_http::auth::{
    AsyncRequireAuthorization, AsyncRequireAuthorizationLayer,
};

use crate::{middleware::service::UserAuthorize, roles::UserRoleVerify};

type InnerLayer<L> = AsyncRequireAuthorizationLayer<UserAuthorize<L>>;

#[derive(Clone)]
pub struct AuthorizeLayer<L: UserRoleVerify>(InnerLayer<L>);

impl<L: UserRoleVerify> Default for AuthorizeLayer<L> {
    fn default() -> Self { Self::new() }
}

impl<S, L> Layer<S> for AuthorizeLayer<L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: UserRoleVerify,
{
    type Service = AsyncRequireAuthorization<S, UserAuthorize<L>>;

    fn layer(&self, inner: S) -> Self::Service { self.0.layer(inner) }
}

impl<L: UserRoleVerify> AuthorizeLayer<L> {
    pub fn new() -> Self { Self(InnerLayer::new(UserAuthorize::default())) }
}
