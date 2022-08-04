use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Future;
use pin_project::pin_project;

use crate::Checker;

#[derive(Debug)]
#[pin_project(project = EnumCheckFut)]
pub enum CheckFut<C: Checker> {
    Fut(#[pin] C::Fut),
    Checked(Option<C::Checked>),
}

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
            EnumCheckFut::Fut(fut) => {
                match fut.poll(cx) {
                    Poll::Ready(Ok(checked)) => {
                        self.set(Self::Checked(checked.into()));
                        Poll::Pending
                    }
                    Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                    Poll::Pending => Poll::Pending,
                }
            }
            // the future is finish, will always return ready
            EnumCheckFut::Checked(_) => Poll::Ready(Ok(())),
        }
    }
}
