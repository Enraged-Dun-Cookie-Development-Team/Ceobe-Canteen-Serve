use axum::{body::Body, response::Response};
use http::Request;
use tower::{Layer, Service};
use tower_http::auth::{
    AsyncRequireAuthorization, AsyncRequireAuthorizationLayer,
};

use crate::author_verify::{service::UserAuthorize, AuthorVerifier};

type InnerLayer<L> = AsyncRequireAuthorizationLayer<UserAuthorize<L>>;

#[derive(Clone)]
pub struct AuthorizeLayer<L>(InnerLayer<L>)
where
    L: AuthorVerifier;

impl<L> AuthorizeLayer<L>
where
    L:  AuthorVerifier + Default
{
    pub fn new()->Self{
        Self(InnerLayer::new(UserAuthorize::new(L::default())))
    }
}

impl<S, L> Layer<S> for AuthorizeLayer<L>
where
    L: AuthorVerifier + Clone,
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    type Service = AsyncRequireAuthorization<S, UserAuthorize<L>>;

    fn layer(&self, inner: S) -> Self::Service { self.0.layer(inner) }
}
