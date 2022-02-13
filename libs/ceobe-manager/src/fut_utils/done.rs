use actix::{Actor, ActorFuture, AsyncContext, Context};

use super::ExecutableFut;

pub struct Done<F, A>(pub(super) F)
where
    F: ActorFuture<Actor = A, Output = ()> + 'static,
    A: Actor<Context = Context<A>>;

impl<F, A> ExecutableFut<A> for Done<F, A>
where
    F: ActorFuture<Actor = A, Output = ()> + 'static,
    A: Actor<Context = Context<A>>,
{
    #[inline]
    fn exec(self, ctx: &mut Context<A>) -> actix::SpawnHandle {
        ctx.spawn(self.0)
    }
}
