use std::future::Future;

use serde::Serialize;

use super::EncodeError;

pub struct Encoder<S>(pub(crate) S);

impl<S> Encoder<S> {
    pub fn new(inner: S) -> Self { Self(inner) }
}

impl<S, Req> tower::Service<Req> for Encoder<S>
where
    S: tower::Service<Req>,
    S::Response: Serialize,
{
    type Error = EncodeError<S::Error>;
    type Response = Vec<u8>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self, cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(EncodeError::Inner)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let fut = self.0.call(req);

        async move {
            let resp = fut.await.map_err(EncodeError::Inner)?;

            let res =
                bincode::serialize(&resp).map_err(EncodeError::Encode)?;

            Ok(res)
        }
    }
}
