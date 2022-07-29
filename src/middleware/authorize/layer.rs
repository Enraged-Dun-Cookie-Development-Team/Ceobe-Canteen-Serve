use std::marker::PhantomData;

use axum::{body::Body, response::Response};
use http::Request;
use tower::{Layer, Service};

use super::AuthorizeService;
use crate::utils::user_authorize::auth_level::AuthLevelVerify;

pub struct AuthorizeLayer<L: AuthLevelVerify>(PhantomData<L>);

impl<S, L> Layer<S> for AuthorizeLayer<L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    type Service = AuthorizeService<S, L>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizeService {
            inner,
            _pha: PhantomData,
        }
    }
}

impl<L: AuthLevelVerify> AuthorizeLayer<L> {
    pub fn new() -> Self { Self(PhantomData) }
}
