use axum::body::Body;
use axum::response::Response;
use http::Request;
use tower::{Layer, Service};
use tower_http::auth::{AsyncRequireAuthorization, AsyncRequireAuthorizationLayer};
use crate::middleware::service::UserAuthorize;
use crate::roles::UserRoleVerify;

type InnerLayer<L> = AsyncRequireAuthorizationLayer<UserAuthorize<L>>;

pub struct AuthorizeLayer<L:UserRoleVerify>(InnerLayer<L>);

impl<S, L> Layer<S> for AuthorizeLayer<L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L:UserRoleVerify,
{
    type Service = AsyncRequireAuthorization<S, UserAuthorize<L>>;

    fn layer(&self, inner: S) -> Self::Service { self.0.layer(inner) }
}

impl<L: UserRoleVerify> AuthorizeLayer<L> {
    pub fn new() -> Self { Self(InnerLayer::new(UserAuthorize::default())) }
}