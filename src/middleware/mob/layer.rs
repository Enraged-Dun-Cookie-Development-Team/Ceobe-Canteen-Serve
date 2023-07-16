use axum::{body::Body, response::Response};
use http::Request;
use tower::{Layer, Service};
use tower_http::auth::{
    AsyncRequireAuthorization, AsyncRequireAuthorizationLayer,
};

use super::service::{self, MobVerify};

#[derive(Clone)]
pub struct MobVerifyLayer(InnerLayer);

impl<S> Layer<S> for MobVerifyLayer
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    type Service = AsyncRequireAuthorization<S, MobVerify>;

    fn layer(&self, inner: S) -> Self::Service { self.0.layer(inner) }
}

impl MobVerifyLayer {
    #[allow(dead_code)]
    pub fn new() -> Self { Self(InnerLayer::new(MobVerify)) }
}

type InnerLayer = AsyncRequireAuthorizationLayer<service::MobVerify>;
