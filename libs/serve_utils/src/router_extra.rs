use axum::Router;
use crate::ControllerRouter;

pub trait ControllerRouterExt<C>{
    fn nest_controller(self,controller:C)->Self;
    
    fn merge_controller(self,controller:C)->Self;
    
}

impl <C,S> ControllerRouterExt<C> for Router<S>
where
    C: ControllerRouter<S>,
    S: Send + Sync + 'static + Clone
{
    fn nest_controller(self, controller: C) -> Self {
        self.nest(C::BASE_URI,<C as ControllerRouter<S>>::route(controller))
    }

    fn merge_controller(self, controller: C) -> Self {
        self.merge(<C as ControllerRouter<S>>::route(controller))
    }
}