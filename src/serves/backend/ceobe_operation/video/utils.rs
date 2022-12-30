use std::{convert::Infallible, time::Duration};

use ammonia::Url;
use axum::{
    body::Bytes,
    extract::{FromRef, FromRequestParts},
};
use axum_starter::{prepare, state::AddState};
use futures::{future::ok, TryFutureExt};
use orm_migrate::sql_models::ceobe_operation::video::checkers::bv::Bv;
use status_err::ErrPrefix;
use tokio::sync::{mpsc, oneshot};

#[prepare(box BiliClient?)]
pub fn prepare_bili_client() -> Result<AddState<QueryBiliVideo>, PrepareError>
{
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) \
         Gecko/20100101 Firefox/102.0",
        )
        .build()?;

    let base_url =
        Url::parse("https://api.bilibili.com/x/web-interface/view")?;
    let (send, mut recv) = mpsc::channel::<(
        Bv,
        oneshot::Sender<Result<Bytes, reqwest::Error>>,
    )>(8);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        while let Some((bv, feedback)) = recv.recv().await {
            let mut url = base_url.clone();
            url.query_pairs_mut()
                .clear()
                .append_pair("bvid", bv.as_str());
            let v =
                client.get(url).send().and_then(|resp| resp.bytes()).await;

            feedback.send(v).ok();
            interval.tick().await;
        }
    });

    Ok(AddState::new(QueryBiliVideo { sender: send }))
}

#[derive(Debug, thiserror::Error)]
pub enum PrepareError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}

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
    pub async fn get_bili_video(
        &self, bv: Bv,
    ) -> Result<Result<Bytes, reqwest::Error>, ChannelClose> {
        let (rx, tx) = oneshot::channel();

        self.sender.send((bv, rx)).await.map_err(|_| ChannelClose)?;

        tx.await.map_err(|_| ChannelClose)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("管道发生未预期关闭")]
pub struct ChannelClose;

impl status_err::StatusErr for ChannelClose {
    fn prefix(&self) -> status_err::ErrPrefix {
        ErrPrefix::SERVE
    }

    fn code(&self) -> u16 {
        0x0002
    }
}
