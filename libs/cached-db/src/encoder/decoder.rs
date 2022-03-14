use std::{future::Future, marker::PhantomData};

use serde::Deserialize;

use super::EncodeError;

pub struct Decoder<S>(pub(crate) S);

impl<S> Decoder<S> {
    pub fn new(inner: S) -> Self {
        Self(inner)
    }
}

pub struct DecodeReq<Req, T>(Req, PhantomData<T>);

impl<Req, T> DecodeReq<Req, T> {
    pub fn new(req: Req) -> Self {
        Self(req, Default::default())
    }
}

impl<S, Req, T> tower::Service<DecodeReq<Req, T>> for Decoder<S>
where
    S: tower::Service<Req>,
    S::Response: AsRef<[u8]>,
    T: for<'de> Deserialize<'de>,
{
    type Response = T;

    type Error = EncodeError<S::Error>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(EncodeError::Inner)
    }

    fn call(&mut self, DecodeReq(req, _): DecodeReq<Req, T>) -> Self::Future {
        let task = self.0.call(req);

        async move {
            let sl = task.await.map_err(EncodeError::Inner)?;
            let data: T = bincode::deserialize(sl.as_ref()).map_err(EncodeError::Encode)?;

            Ok(data)
        }
    }
}
