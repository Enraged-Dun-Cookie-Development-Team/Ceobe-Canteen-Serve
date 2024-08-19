use std::{convert::Infallible, marker::PhantomData};

use axum::{routing::Route, Router};
use axum_core::{extract::Request, response::IntoResponse};
use axum_resp_result::{MapReject, ToInner};
use tower::{Layer, Service};

pub trait ControllerRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    const BASE_URI: &'static str;

    fn route(self) -> Router<S> { Router::new() }

    fn layer<L: Clone>(self, layer: L) -> LayeredController<Self, S>
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        LayeredController::new(ControllerRouter::route(self).layer(layer))
    }
}

pub struct LayeredController<C, S> {
    inner: Router<S>,
    __phantom: PhantomData<C>,
}

impl<C, S> LayeredController<C, S> {
    fn new(router: Router<S>) -> Self {
        Self {
            inner: router,
            __phantom: Default::default(),
        }
    }
}

impl<S, C> ControllerRouter<S> for LayeredController<C, S>
where
    S: Clone + Send + Sync + 'static,
    C: ControllerRouter<S>,
{
    const BASE_URI: &'static str = <C as ControllerRouter<S>>::BASE_URI;

    fn route(self) -> Router<S> { self.inner }
}

pub trait ControllerError {
    type Error;
}

pub type HandlerResult<T, C: ControllerError> =
    Result<T, <C as ControllerError>::Error>;
pub type HandlerMapReject<T: ToInner, C: ControllerError> =
    MapReject<T, <C as ControllerError>::Error>;
