use std::future::Future;

use futures::future::Ready;

pub trait SyncFuture:Future {
    fn into_inner(self)->Self::Output;
}

impl<T> SyncFuture for Ready<T> {
    fn into_inner(self)->Self::Output {
        self.into_inner()
    }
}