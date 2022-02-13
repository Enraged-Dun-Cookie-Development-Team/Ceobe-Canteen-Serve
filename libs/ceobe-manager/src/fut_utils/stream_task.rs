use actix::{
    fut::{wrap_future, FutureWrap, Map},
    Actor, ActorFuture, Context,
};
use futures_util::Future;

use super::done::Done;

pub struct FutureTask<F, A>(pub(super) FutureWrap<F, A>)
where
    F: Future,
    A: Actor<Context = Context<A>>;

impl<F, A> FutureTask<F, A>
where
    F: Future,
    A: Actor<Context = Context<A>>,
{
    #[inline]
    pub fn start(fut: F) -> Self {
        Self(wrap_future(fut))
    }
    #[inline]
    pub fn then<Func, F2>(self, handle: Func) -> super::map::Map<FutureWrap<F, A>, Func, A, F2>
    where
        Func: 'static,

        Func: FnOnce(F::Output, &mut A, &mut A::Context) -> F2,
    {
        super::map::Map(self.0.map(handle))
    }
    #[inline]
    pub fn done<Func>(self, handle: Func) -> Done<Map<FutureWrap<F, A>, Func>, A>
    where
        Func: 'static,
        Func: FnOnce(F::Output, &mut A, &mut A::Context),
    {
        let req = self.0.map(handle);
        Done(req)
    }
}
