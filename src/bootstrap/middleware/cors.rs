use axum_starter::{prepare, PrepareMiddlewareEffect};
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub trait CorsConfigTrait {
    fn allow_origins(&self) -> Vec<HeaderValue>;

    fn allow_methods(&self) -> Vec<Method>;
}

pub struct CorsConfig {
    origins: Vec<HeaderValue>,
    methods: Vec<Method>,
}

#[prepare(PrepareCors)]
pub fn prepare_cors<T: CorsConfigTrait>(cfg: &T) -> CorsMiddleware {
    let config = CorsConfig {
        origins: cfg.allow_origins(),
        methods: cfg.allow_methods(),
    };
    CorsMiddleware(config)
}

pub struct CorsMiddleware(CorsConfig);

impl<S> PrepareMiddlewareEffect<S> for CorsMiddleware {
    type Middleware = CorsLayer;

    fn take(self, _: &mut axum_starter::StateCollector) -> Self::Middleware {
        CorsLayer::new()
            .allow_origin(self.0.origins)
            .allow_methods(self.0.methods)
    }
}
