use axum_starter::{PrepareMiddlewareEffect, prepare};
use tower_http::{
    LatencyUnit,
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{
        DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer,
    },
};
use tracing::Level;

#[prepare(PrepareRequestTracker)]
pub fn prepare_track_request() -> RequestTracker { RequestTracker }

pub struct RequestTracker;

impl<S> PrepareMiddlewareEffect<S> for RequestTracker {
    type Middleware = TraceLayer<SharedClassifier<ServerErrorsAsFailures>>;

    fn take(self, _: &mut axum_starter::StateCollector) -> Self::Middleware {
        tracing_request()
    }
}

pub fn tracing_request()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
}
