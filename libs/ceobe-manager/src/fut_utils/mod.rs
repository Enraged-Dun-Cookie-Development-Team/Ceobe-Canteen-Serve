mod done;
mod map;
mod stream_task;

use std::fmt::Debug;

use actix::{fut::wrap_future, Actor, ActorFuture, AsyncContext, Context, SpawnHandle};
use futures_util::Future;

#[inline]
pub(crate) fn do_feature<F, A>(fut: F, ctx: &mut Context<A>) -> SpawnHandle
where
    F: Future + 'static,
    F::Output: Debug,
    A: Actor<Context = Context<A>>,
{
    let task = async move {
        let _r = fut.await;
        eprintln!("Result `{:?}`", _r);
        ()
    };
    let wrap = wrap_future(task);

    ctx.spawn(wrap)
}
#[inline]
pub(crate) fn do_with_func<F, A, H>(fut: F, ctx: &mut Context<A>, handle: H) -> SpawnHandle
where
    F: Future + 'static,
    A: Actor<Context = Context<A>>,
    H: FnOnce(F::Output, &mut A, &mut A::Context) + 'static,
{
    let wrap = wrap_future::<F, A>(fut);
    let task = wrap.map(handle);

    ctx.spawn(task)
}

pub trait ExecutableFut<A>
where
    A: Actor<Context = Context<A>>,
{
    fn exec(self, ctx: &mut Context<A>) -> SpawnHandle;
}

pub use done::Done;pub use map::Map as TaskMap;
pub use stream_task::FutureTask;
