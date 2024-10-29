use std::{convert::Infallible, marker::PhantomData};

use axum::{
    extract::Request, response::IntoResponse, routing::Route, Router,
};
use tower::{Layer, Service};

use crate::{ControllerRoute, EndpointType};

pub trait ControllerRouterExt<C, E> {
    fn nest_controller(self, controller: C, _endpoint: E) -> Self;

    fn merge_controller(self, controller: C, _endpoint: E) -> Self;
}

impl<C, S, E> ControllerRouterExt<C, E> for Router<S>
where
    C: ControllerRoute<S, E>,
    E: EndpointType,
    S: Send + Sync + 'static + Clone,
{
    fn nest_controller(self, controller: C, _endpoint: E) -> Self {
        self.nest(
            C::BASE_URI,
            <C as ControllerRoute<S, E>>::route(controller),
        )
    }

    fn merge_controller(self, controller: C, _endpoint: E) -> Self {
        self.merge(<C as ControllerRoute<S, E>>::route(controller))
    }
}

pub struct ControllerRouter<S, E> {
    router: Router<S>,
    __phantom: PhantomData<E>,
}

impl<S, E> Into<Router<S>> for ControllerRouter<S, E> {
    fn into(self) -> Router<S> {
        self.router
    }
}

impl<S, E> ControllerRouter<S, E>
where
    E: EndpointType,
    S: Send + Sync + 'static + Clone,
{
    pub fn nest_controller<C: ControllerRoute<S, E>>(
        self, controller: C,
    ) -> Self {
        self.nest(
            C::BASE_URI,
            <C as ControllerRoute<S, E>>::route(controller),
        )
    }

    pub fn merge_controller<C: ControllerRoute<S, E>>(
        self, controller: C,
    ) -> Self {
        let router = self
            .router
            .merge(<C as ControllerRoute<S, E>>::route(controller));
        Self { router, ..self }
    }

    pub fn nest(self, path: &str, handler: Router<S>) -> Self {
        let router = self.router.nest(path, handler);
        Self { router, ..self }
    }

    pub fn merge<R: Into<Router<S>>>(self, handler: R) -> Self {
        let router = self.router.merge(handler);
        Self { router, ..self }
    }

    pub fn route_layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        let router = self.router.route_layer(layer);
        Self { router, ..self }
    }
}

impl<S, E> ControllerRouter<S, E> {
    pub fn new(_endpoint: E) -> Self
    where
        S: Clone + Send + Sync + 'static,
        E: EndpointType,
    {
        Self {
            router: Router::new(),
            __phantom: PhantomData,
        }
    }

    pub fn set_router<F: FnOnce(Router<S>) -> Router<S>>(
        self, handle: F,
    ) -> Self {
        let router = handle(self.router);
        Self { router, ..self }
    }
}

#[cfg(test)]
mod test {
    use axum::Router;

    use crate::{
        endpoint::AdminEnd, endpoint_type::UserEnd, ControllerRoute,
        ControllerRouterExt,
    };

    pub struct TestController;

    impl<S: Send + Sync + 'static + Clone> ControllerRoute<S, AdminEnd>
        for TestController
    {
        const BASE_URI: &'static str = "/admin/test";
    }

    impl<S: Send + Sync + 'static + Clone> ControllerRoute<S, UserEnd>
        for TestController
    {
        const BASE_URI: &'static str = "/test";
    }
    #[test]
    fn test_ctrl() {
        let _:Router = Router::new()
            .nest_controller(TestController,UserEnd)
            .nest_controller(TestController,AdminEnd)
            // .nest_controller(TestController,CDN)
            ;
    }
}
