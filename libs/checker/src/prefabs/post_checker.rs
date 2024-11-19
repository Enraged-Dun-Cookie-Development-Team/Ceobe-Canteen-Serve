use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Future};
use pin_project::pin_project;

use crate::{sync_check::SyncFuture, Checker};

/// 追加checker
///
/// 执行完成 `C::check` 后使用 `P::ref_check` 进行追加checker
pub struct PostChecker<C, P, E>(PhantomData<(C, P, E)>)
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>;

impl<C, P, E> Checker for PostChecker<C, P, E>
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    type Args = (C::Args, P::Args);
    type Checked = P::Checked;
    type Err = E;
    type Fut = PostCheckFut<C, P, E>;
    type Unchecked = C::Unchecked;

    fn check(
        (c_args, p_args): Self::Args, uncheck: Self::Unchecked,
    ) -> Self::Fut {
        PostCheckFut::new(C::check(c_args, uncheck), p_args)
    }
}

#[pin_project(project = ProjEnum)]
pub enum PostCheckFut<C, P, E>
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    Checker(#[pin] C::Fut, Option<P::Args>, PhantomData<E>),
    PostChecker(#[pin] <P as Checker>::Fut),
}

impl<C, P, E> SyncFuture for PostCheckFut<C, P, E>
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
    C::Fut: SyncFuture,
    P::Fut: SyncFuture,
{
    fn into_inner(self) -> Self::Output {
        match self {
            PostCheckFut::Checker(fut, Some(args), _) => {
                let mid = SyncFuture::into_inner(fut)?;
                let p_fut = P::check(args, mid);
                let out = SyncFuture::into_inner(p_fut)?;
                Ok(out)
            }
            _ => unreachable!(),
        }
    }
}

impl<C, P, E> PostCheckFut<C, P, E>
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    pub fn new(fut: C::Fut, args: P::Args) -> Self {
        Self::Checker(fut, Some(args), PhantomData)
    }
}

impl<C, P, E> Future for PostCheckFut<C, P, E>
where
    C: Checker,
    P: Checker<Unchecked = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    type Output = Result<P::Checked, E>;

    fn poll(
        mut self: Pin<&mut Self>, cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.as_mut().project();

        match this {
            ProjEnum::Checker(fut, p_arg, _) => {
                // poll first
                let result = ready!(fut.poll(cx)).map_err(E::from);
                match result {
                    Ok(checked) => {
                        // create next step future
                        let mut fut = P::check(
                            p_arg.take().expect("Poll Ready Future"),
                            checked,
                        );
                        // pinned it
                        let pined_fut =
                            unsafe { Pin::new_unchecked(&mut fut) };
                        // poll it
                        let Poll::Ready(result) = pined_fut.poll(cx)
                        else {
                            // if pending update self
                            self.set(PostCheckFut::PostChecker(fut));
                            // keep pending
                            return Poll::Pending;
                        };
                        // if ready return
                        Poll::Ready(result.map_err(E::from))
                    }
                    Err(err) => Poll::Ready(Err(err)),
                }
            }
            ProjEnum::PostChecker(fut) => {
                // poll until ready
                Poll::Ready(ready!(fut.poll(cx)).map_err(E::from))
            }
        }
    }
}
