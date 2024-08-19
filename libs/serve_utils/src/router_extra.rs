use axum::Router;

use crate::{ControllerRouter, EndpointType};

pub trait ControllerRouterExt<C, E> {
    fn nest_controller(self, controller: C, _endpoint: E) -> Self;

    fn merge_controller(self, controller: C, _endpoint: E) -> Self;
}

impl<C, S, E> ControllerRouterExt<C, E> for Router<S>
where
    C: ControllerRouter<S, E>,
    E: EndpointType,
    S: Send + Sync + 'static + Clone,
{
    fn nest_controller(self, controller: C, _endpoint: E) -> Self {
        self.nest(
            C::BASE_URI,
            <C as ControllerRouter<S, E>>::route(controller),
        )
    }

    fn merge_controller(self, controller: C, _endpoint: E) -> Self {
        self.merge(<C as ControllerRouter<S, E>>::route(controller))
    }
}

#[cfg(test)]
mod test {
    use axum::Router;

    use crate::{
        endpoint::AdminEnd, endpoint_type::UserEnd, ControllerRouter,
        ControllerRouterExt,
    };

    pub struct TestController;

    impl<S: Send + Sync + 'static + Clone> ControllerRouter<S, AdminEnd>
        for TestController
    {
        const BASE_URI: &'static str = "/admin/test";
    }

    impl<S: Send + Sync + 'static + Clone> ControllerRouter<S, UserEnd>
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
