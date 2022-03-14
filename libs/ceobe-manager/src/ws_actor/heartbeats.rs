use std::{ task::Poll, time::Duration};

use actix::StreamHandler;
use futures_util::{ pin_mut, Stream, Future};

use super::CeoboWebsocket;

#[pin_project::pin_project]
pub(super) struct HeartBeats {
    inner: actix_rt::time::Interval,
}

pub(super) struct Beating;

impl Stream for HeartBeats {
    type Item = Beating;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
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
        #[cfg(feature = "log")]
        log_::info!("发送心跳包");
        match self.slink.write(actix_http::ws::Message::Ping("heart beating!".into())) {
            Ok(()) => {},
            Err(_er) => {
                #[cfg(feature = "log")]
                log_::error!("无法发送心跳包",)
            },
        }
    }
}