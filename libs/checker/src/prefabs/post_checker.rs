use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Future};
use pin_project::pin_project;

use crate::{Checker, RefChecker};
/// 追加checker
/// 
/// 执行完成 `C::check` 后使用 `P::ref_check` 进行追加checker
pub struct PostChecker<C, P, E>(PhantomData<(C, P, E)>);

impl<C, P, E> Checker for PostChecker<C, P, E>
where
    C: Checker,
    P: RefChecker<Target = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    type Unchecked = C::Unchecked;

    type Args = (C::Args, P::Args);

    type Checked = C::Checked;

    type Err = E;

    type Fut = PostCheckFut<C, P, E>;

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
    P: RefChecker,
    E: From<C::Err>,
    E: From<P::Err>,
{
    Checker(#[pin] C::Fut, Option<P::Args>, PhantomData<E>),
    PostChecker(#[pin] <P as Checker>::Fut),
}

impl<C, P, E> PostCheckFut<C, P, E>
where
    C: Checker,
    P: RefChecker,
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
    P: RefChecker<Target = C::Checked>,
    E: From<C::Err>,
    E: From<P::Err>,
{
    type Output = Result<C::Checked, E>;

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
                        let Poll::Ready(result) = pined_fut.poll(cx)else{
                            // if pending update self
                            self.set(PostCheckFut::PostChecker(fut));
                            // keep pending
                            return Poll::Pending
                        };
                        // if ready return
                        Poll::Ready(result.map_err(E::from))
                    }
                    err @ Err(_) => Poll::Ready(err),
                }
            }
            ProjEnum::PostChecker(fut) => {
                // poll until ready
                Poll::Ready(ready!(fut.poll(cx)).map_err(E::from))
            }
        }
    }
}
