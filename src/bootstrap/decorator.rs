use std::{future::Future, time::Duration};

use axum_starter::{prepare, PrepareDecorator, PrepareError};
use futures::{future::LocalBoxFuture, FutureExt, TryFutureExt};
use tokio::time::timeout;

#[prepare(sync Decroator)]
fn prepare_decorator() -> TimeoutDecorator {
    TimeoutDecorator(Duration::from_secs(5))
}

pub struct TimeoutDecorator(
    /// 超时时间
    pub Duration,
);

impl PrepareDecorator for TimeoutDecorator {
    type OutFut< Fut, T> = LocalBoxFuture<'static,Result<T,PrepareError>>

        where Fut: Future<Output=Result<T, PrepareError>> +'static,T: 'static;

    fn decorator<Fut, T>(
        &self, src: &'static str, in_fut: Fut,
    ) -> Self::OutFut<Fut, T>
    where
        Fut: Future<Output = Result<T, PrepareError>> + 'static,
        T: 'static,
    {
        let duration = self.0;
        Box::pin(
            timeout(duration, in_fut)
                .map_err(move |_err| {
                    PrepareError::new(src, Box::new(Error(duration)))
                })
                .map(|output| {
                    match output {
                        Ok(data @ Ok(_)) => data,
                        Ok(Err(err)) | Err(err) => Err(err),
                    }
                }),
        )
    }
}
use thiserror::Error as DeriveError;
#[derive(DeriveError, Debug)]
#[error("deadline[{0:?}] has elapsed")]
pub struct Error(Duration);
