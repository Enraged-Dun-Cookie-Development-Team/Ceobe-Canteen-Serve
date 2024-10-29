use std::{convert::Infallible, marker::PhantomData};

use axum::{routing::Route, Router};
use axum_core::{extract::Request, response::IntoResponse};
use axum_resp_result::MapReject;
use tower::{Layer, Service};

use crate::EndpointType;

pub trait ControllerRoute<S, E>
where
    S: Clone + Send + Sync + 'static,
    E: EndpointType,
    Self: Sized,
{
    const BASE_URI: &'static str;

    fn route(self) -> Router<S> { Router::new() }

    fn layer<L>(self, layer: L) -> LayeredController<Self, S>
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        LayeredController::new(ControllerRoute::route(self).layer(layer))
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

impl<S, C, E> ControllerRoute<S, E> for LayeredController<C, S>
where
    S: Clone + Send + Sync + 'static,
    C: ControllerRoute<S, E>,
    E: EndpointType,
{
    const BASE_URI: &'static str = <C as ControllerRoute<S, E>>::BASE_URI;

    fn route(self) -> Router<S> { self.inner }
}

pub trait ControllerError {
    type Error;
}

pub type HandlerResult<T, C> = Result<T, <C as ControllerError>::Error>;
pub type HandlerMapReject<T, C> = MapReject<T, <C as ControllerError>::Error>;
