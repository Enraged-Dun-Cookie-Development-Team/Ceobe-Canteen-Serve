use std::{
    task::Poll,
    time::{Duration, SystemTime},
};

use actix::StreamHandler;
use futures_util::{pin_mut, Future, Stream};

use super::CeoboWebsocket;

#[pin_project::pin_project]
pub(super) struct HeartBeats {
    inner: actix_rt::time::Interval,
}

pub(super) struct BeatTimeout<const TIMEOUT_SEC: u64> {
    last: Option<SystemTime>,
}

impl<const TIMEOUT_SEC: u64> Default for BeatTimeout<TIMEOUT_SEC> {
    fn default() -> Self { Self { last: None } }
}

impl<const TIMEOUT_SEC: u64> BeatTimeout<TIMEOUT_SEC> {
    pub(super) fn check_timeout(&mut self) -> bool {
        if let Some(last) = self.last.take() {
            let now = SystemTime::now();
            now.duration_since(last).unwrap()
                <= Duration::from_secs(TIMEOUT_SEC)
        }
        else {
            true
        }
    }

    fn reset(&mut self) { self.last = Some(SystemTime::now()) }
}

pub(super) struct Beating;

impl Stream for HeartBeats {
    type Item = Beating;

    fn poll_next(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let interval = this.inner;
        let tick = interval.tick();
        pin_mut!(tick);
        match tick.poll(cx) {
            Poll::Ready(_) => Poll::Ready(Some(Beating)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl HeartBeats {
    pub(super) fn new() -> Self {
        Self {
            inner: actix_rt::time::interval(Duration::from_secs(5)),
        }
    }
}

impl StreamHandler<Beating> for CeoboWebsocket {
    fn handle(&mut self, _item: Beating, _ctx: &mut Self::Context) {
        // 发送前，检查上一次是否超时

        if !self.beat_timeout.check_timeout() {
            #[cfg(feature = "log")]
            log_::error!("心跳包接收超时");
            self.slink.close();
        }

        #[cfg(feature = "log")]
        log_::debug! ("发送心跳包");
        match self
            .slink
            .write(actix_http::ws::Message::Ping("heart beating!".into()))
        {
            Ok(()) => {}
            Err(_er) => {
                #[cfg(feature = "log")]
                log_::error!("无法发送心跳包");
                self.slink.close()
            }
        }

        self.beat_timeout.reset();
    }
}
