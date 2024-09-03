use std::{collections::HashSet, sync::Arc};

use axum_starter::{prepare, PrepareMiddlewareEffect};
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use std::{task::Poll};

use tower::{Layer, Service};
use tower_http::cors::{Any, Cors};


pub trait CorsConfigTrait {
    fn allow_origins(&self) -> Vec<HeaderValue>;

    fn allow_methods(&self) -> Vec<Method>;

    fn bypass_paths(&self) -> Arc<HashSet<String>>;
}

#[derive(Clone)]
pub struct ConditionCors<S> {
    bypass_cors: Cors<S>,
    default_cors: Cors<S>,
    bypass_paths: Arc<HashSet<String>>,
}

impl<S, Req, Resp> Service<http::Request<Req>> for ConditionCors<S>
where
    S: Service<http::Request<Req>, Response = http::Response<Resp>>,
    Req: Default,
    Resp: Default,
{
    type Response = <Cors<S> as Service<http::Request<Req>>>::Response;

    type Error = <Cors<S> as Service<http::Request<Req>>>::Error;

    type Future = <Cors<S> as Service<http::Request<Req>>>::Future;

    fn poll_ready(
        &mut self, cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        match (
            self.bypass_cors.poll_ready(cx),
            self.default_cors.poll_ready(cx),
        ) {
            (Poll::Ready(bypass), Poll::Ready(default)) => {
                Poll::Ready(bypass.and_then(|_| default))
            }
            (Poll::Ready(_), Poll::Pending) => Poll::Pending,
            (Poll::Pending, Poll::Ready(_)) => Poll::Pending,
            (Poll::Pending, Poll::Pending) => Poll::Pending,
        }
    }

    fn call(&mut self, req: http::Request<Req>) -> Self::Future {
        let uri = req.uri().path();
        if self.bypass_paths.contains(uri) {
            self.bypass_cors.call(req)
        } else {
            self.default_cors.call(req)
        }
    }
}

#[derive(Clone)]
pub struct ConditionCorsLayer {
    bypass_cors: CorsLayer,
    default_cors: CorsLayer,
    bypass_paths: Arc<HashSet<String>>
}

impl<S: Clone> Layer<S> for ConditionCorsLayer {
    type Service = ConditionCors<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConditionCors {
            bypass_cors: self.bypass_cors.layer(inner.clone()),
            default_cors: self.default_cors.layer(inner),
            bypass_paths: self.bypass_paths.clone(),
        }
    }
}

pub struct ConditionCorsMiddlewarePrepare<T: CorsConfigTrait>(T);

impl<S: Clone, T: CorsConfigTrait + 'static> PrepareMiddlewareEffect<S>
    for ConditionCorsMiddlewarePrepare<T>
{
    type Middleware = ConditionCorsLayer;

    fn take(self, _: &mut axum_starter::StateCollector) -> Self::Middleware {
        ConditionCorsLayer {
            bypass_cors: CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any),
            default_cors: CorsLayer::new()
                .allow_origin(self.0.allow_origins())
                .allow_methods(self.0.allow_methods()),
            bypass_paths: self.0.bypass_paths(),
        }
    }
}

#[prepare(ConditionCorsPrepare)]
pub fn prepare_condition_cors<T: CorsConfigTrait + Clone>(
    config: &T,
) -> ConditionCorsMiddlewarePrepare<T> {
    ConditionCorsMiddlewarePrepare(config.clone())
}
