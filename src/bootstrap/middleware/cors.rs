use axum_starter::{prepare, PrepareMiddlewareEffect};
use http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub trait CorsConfigTrait {
    fn allow_origins(&self) -> Vec<HeaderValue>;

    fn allow_methods(&self) -> Vec<Method>;
}

#[prepare(PrepareCors)]
pub fn prepare_cors<T: CorsConfigTrait>(cfg: &T) -> CorsMiddleware {
    CorsMiddleware(
        CorsLayer::new()
            .allow_origin(cfg.allow_origins())
            .allow_methods(cfg.allow_methods()),
    )
}

pub struct CorsMiddleware(CorsLayer);

impl<S> PrepareMiddlewareEffect<S> for CorsMiddleware {
    type Middleware = CorsLayer;

    fn take(self, _: &mut axum_starter::StateCollector) -> Self::Middleware {
        self.0
    }
}
