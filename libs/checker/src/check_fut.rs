use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;
use pin_project::pin_project;

use crate::{Checker, SyncFuture};

#[derive(Debug)]
#[pin_project(project = EnumCheckFut)]
pub enum CheckFut<C: Checker> {
    Fut(#[pin] C::Fut),
    Checked(Option<C::Checked>),
}

impl<C: Checker> CheckFut<C>
where
    C::Fut: SyncFuture,
{
    pub fn into_inner(&mut self) -> Result<C::Checked, C::Err> {
        let this = std::mem::replace(self, Self::Checked(None));
        match this {
            CheckFut::Fut(fut) => SyncFuture::into_inner(fut),
            _ => unreachable!(),
        }
    }
}

// impl<C> SyncFuture for CheckFut<C> where C: Checker ,C::Fut:SyncFuture{
//     fn into_inner(self)->Self::Output {
//         match self{
//             CheckFut::Fut(fut) => todo!(),
//             CheckFut::Checked(_) => todo!(),
//         }
//     }
// }

impl<C: Checker> CheckFut<C> {
    pub fn is_finish(&self) -> bool {
        match self {
            CheckFut::Fut(_) => false,
            CheckFut::Checked(_) => true,
        }
    }

    pub fn take(&mut self) -> C::Checked {
        match self {
            CheckFut::Fut(_) => panic!("that Future Not finish yet"),
            CheckFut::Checked(data) => {
                data.take().expect("The Checked Data has been take")
            }
        }
    }
}

impl<C: Checker> Future for CheckFut<C> {
    type Output = Result<(), C::Err>;

    fn poll(
        mut self: Pin<&mut Self>, cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        match self.as_mut().project() {
            EnumCheckFut::Fut(fut) => match fut.poll(cx) {
                Poll::Ready(Ok(checked)) => {
                    self.set(Self::Checked(checked.into()));
                    Poll::Ready(Ok(()))
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            },
            // the future is finish, will always return ready
            EnumCheckFut::Checked(_) => Poll::Ready(Ok(())),
        }
    }
}
