use std::convert::Infallible;

use axum_core::extract::{FromRef, FromRequestParts};
use bytes::Bytes;
use futures::future::ok;
use persistence::ceobe_operate::models::video::bv;
use tokio::sync::{mpsc, oneshot};

use crate::error::ChannelClose;

/// 获取bili 视频信息支持类型
#[derive(Debug, Clone)]
pub struct QueryBiliVideo {
    sender: mpsc::Sender<(
        bv::Checked,
        oneshot::Sender<Result<Bytes, reqwest::Error>>,
    )>,
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
            bv::Checked,
            oneshot::Sender<Result<Bytes, reqwest::Error>>,
        )>,
    ) -> Self {
        Self { sender }
    }

    /// 给定 BV 号，通过BV号获取对应的视频信息，
    /// 该接口500ms内多次执行将会阻塞到每500ms发送一次请求
    ///
    /// # Errors
    ///
    /// This function will return an error if
    /// 1. 独立协程管道非正常关闭
    /// 2. [reqwest::Error] 请求异常
    pub async fn fetch(
        &self, bv: bv::Checked,
    ) -> Result<Result<Bytes, reqwest::Error>, ChannelClose> {
        let (rx, tx) = oneshot::channel();
        // 将bv 和 回调 一起发送
        self.sender.send((bv, rx)).await.map_err(|_| ChannelClose)?;
        // 等待回调返回
        tx.await.map_err(|_| ChannelClose)
    }
}
