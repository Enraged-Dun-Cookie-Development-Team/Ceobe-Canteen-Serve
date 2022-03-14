use actix::{fut::wrap_future, Actor, ActorFutureExt, Context, SpawnHandle};
use futures_util::Future;

use actix::AsyncContext;

pub(crate) fn do_fut<F, A>(fut: F, ctx: &mut Context<A>) -> SpawnHandle
where
    F: Future + 'static,
    A: Actor<Context = Context<A>>,
{
    let task = wrap_future(async move {
        fut.await;
        ()
    });
    ctx.spawn(task)
}

pub(crate) fn do_fut_with<F, A, O, H>(fut: F, ctx: &mut Context<A>, handle: H) -> SpawnHandle
where
    F: Future<Output = O> + 'static,
    A: Actor<Context = Context<A>>,
    H: FnOnce(O, &mut A, &mut Context<A>) + 'static,
{
    let task = wrap_future(fut);
    let task = task.map::<_, ()>(handle);
    ctx.spawn(task)
}
