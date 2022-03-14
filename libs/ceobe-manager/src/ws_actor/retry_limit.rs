use std::{fmt::Display, time::Duration};

use actix::{io::SinkWrite, Actor, AsyncContext};
use awc::{error::WsClientError, ClientResponse};

use crate::fut_utils::do_fut_with;

use super::{conn_ws, CeoboWebsocket, WsFramedSink, WsFramedStream};

pub struct RetryLimit<const LIMIT: usize> {
    try_time: usize,
}

impl<const LIMIT: usize> Display for RetryLimit<LIMIT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} / {} ]", self.try_time, LIMIT)
    }
}

impl<const LIMIT: usize> Iterator for RetryLimit<LIMIT> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.try_time <= LIMIT {
            self.try_time += 1;
            Some(())
        } else {
            None
        }
    }
}

impl<const LIMIT: usize> RetryLimit<LIMIT> {
    pub(crate) fn reset(&mut self) {
        self.try_time = 0
    }
}

impl<const LIMIT: usize> Default for RetryLimit<LIMIT> {
    fn default() -> Self {
        Self { try_time: 0 }
    }
}

pub(super) fn retry_result_handle(
    ws_result: Result<(ClientResponse, (WsFramedSink, WsFramedStream)), WsClientError>,
    actor: &mut CeoboWebsocket,
    context: &mut <CeoboWebsocket as Actor>::Context,
) {
    match ws_result {
        Ok((_, (sink, stream))) => {
            context.add_stream(stream);
            actor.slink = SinkWrite::new(sink, context);
        }
        Err(err) => {
            #[cfg(feature = "log")]
            log_::error!("重连尝试时出现异常: {}", err);
            if let Some(()) = actor.retry.next() {
                #[cfg(feature = "log")]
                log_::error!("开始尝试丛连 {}", actor.retry);
                do_fut_with(do_retry(actor.uri), context, retry_result_handle);
            } else {
                #[cfg(feature = "log")]
                log_::warn!("重连次数到达上限，终止尝试");
            }
        }
    }
}

pub(super) async fn do_retry(
    uri: &'static str,
) -> Result<(ClientResponse, (WsFramedSink, WsFramedStream)), WsClientError> {
    // 等待2000ms 后丛连
    actix_rt::time::sleep(Duration::from_millis(2000)).await;
    // 重连
    conn_ws(uri).await
}
