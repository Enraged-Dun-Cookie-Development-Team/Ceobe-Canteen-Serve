use actix::{Actor, ActorFuture, Context};

use super::done::Done;

pub struct Map<F, Func, A, O>(pub(super) actix::fut::Map<F, Func>)
where
    F: ActorFuture<Actor = A>,
    A: Actor<Context = Context<A>>,
    Func: FnOnce(F::Output, &mut A, &mut A::Context) -> O;

impl<F, Func, A, O> Map<F, Func, A, O>
where
    F: ActorFuture<Actor = A>,
    A: Actor<Context = Context<A>>,
    Func: FnOnce(F::Output, &mut A, &mut A::Context) -> O,
{
    #[inline]
    pub fn then<Func2, F2>(self, handle: Func2) -> Map<actix::fut::Map<F, Func>, Func2, A, F2>
    where
        Func2: 'static,
        Func2: FnOnce(O, &mut A, &mut A::Context) -> F2,
    {
        let req = self.0.map(handle);

        Map(req)
    }
    #[inline]
    pub fn done<Func2>(
        self,
        handle: Func2,
    ) -> Done<actix::fut::Map<actix::fut::Map<F, Func>, Func2>, A>
    where
        Func2: 'static,
        Func2: FnOnce(O, &mut A, &mut A::Context),
    {
        let req = self.0.map(handle);
        Done(req)
    }
}
