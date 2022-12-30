use std::convert::Infallible;

use axum_core::extract::{FromRef, FromRequestParts};
use bytes::Bytes;
use futures::future::ok;
use sql_models::ceobe_operation::video::checkers::bv::Bv;

use tokio::sync::{mpsc, oneshot};

use crate::error::ChannelClose;

#[derive(Debug, Clone)]
pub struct QueryBiliVideo {
    sender:
        mpsc::Sender<(Bv, oneshot::Sender<Result<Bytes, reqwest::Error>>)>,
}

impl<S> FromRequestParts<S> for QueryBiliVideo
where
    S: 'static,
    QueryBiliVideo: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _: &'life0 mut http::request::Parts, state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(ok(Self::from_ref(state)))
    }
}

impl QueryBiliVideo {
    pub(super) fn new(
        sender: mpsc::Sender<(
            Bv,
            oneshot::Sender<Result<Bytes, reqwest::Error>>,
        )>,
    ) -> Self {
        Self { sender }
    }

    pub async fn fetch(
        &self, bv: Bv,
    ) -> Result<Result<Bytes, reqwest::Error>, ChannelClose> {
        let (rx, tx) = oneshot::channel();

        self.sender.send((bv, rx)).await.map_err(|_| ChannelClose)?;

        tx.await.map_err(|_| ChannelClose)
    }
}
