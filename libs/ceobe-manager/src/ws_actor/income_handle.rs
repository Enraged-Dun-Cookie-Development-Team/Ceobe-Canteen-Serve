use actix::{ActorContext, StreamHandler};
use awc::error::WsProtocolError;
use awc::ws::Message;

use awc::ws;

use crate::{
    fut_utils::{do_fut, do_fut_with},
    ws_actor::{
        continuation,
        retry_limit::{do_retry, retry_result_handle},
    },
};

use super::CeoboWebsocket;

impl actix::io::WriteHandler<awc::error::WsProtocolError> for CeoboWebsocket {}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for CeoboWebsocket {
    fn started(&mut self, _ctx: &mut Self::Context) {
        #[cfg(feature = "log")]
        log_::info!("ws 连接建立成功");

        self.retry.reset();
    }

    fn handle(&mut self, item: Result<ws::Frame, WsProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(msg) => match msg {
                ws::Frame::Text(text) | ws::Frame::Binary(text) => {
                    #[cfg(feature = "log")]
                    log_::info!("Handling ws Frame Data");
                    do_fut(self.json_handle.send(text.into()), ctx);
                }
                ws::Frame::Continuation(c) => {
                    #[cfg(feature = "log")]
                    log_::info!("Handling ws Split Frame");
                    let req = self.continue_handle.send(continuation::NextIncome(c));

                    do_fut_with(req, ctx, |res, actor, ctx| {
                        if let Ok(Some(msg)) = res {
                            #[cfg(feature = "log")]
                            log_::info!("Split Frame Data clear Handling");
                            do_fut(actor.json_handle.send(msg.unwrap().into()), ctx);
                        }
                    });
                }
                ws::Frame::Ping(p) => {
                    #[cfg(feature = "log")]
                    log_::info!("收到 Ping 消息： `{}`", String::from_utf8_lossy(&p));
                    self.slink.write(Message::Pong(p)).ok();
                }
                ws::Frame::Pong(p) => {
                    #[cfg(feature = "log")]
                    log_::info!("收到 Pong 消息: `{}`", String::from_utf8_lossy(&p));
                    if !self.beat_timeout.check_timeout(){
                        self.slink.close()
                    }
                }
                ws::Frame::Close(c) => {
                    if let Some(reason) = c {
                        #[cfg(feature = "log")]
                        log_::warn!(
                            "Websocket Service Close Connection. \ncode :{:?} `{}`",
                            reason.code,
                            reason.description.unwrap_or_default()
                        );
                        ctx.stop()
                    }
                }
            },
            Err(err) => {
                #[cfg(feature = "log")]
                log_::error!("Websocket Connect Error: `{}`", err);
            }
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        if let Some(()) = self.retry.next() {
            #[cfg(feature = "log")]
            log_::error!("检测到连接断开，正在尝试重连...");
            let uri = self.uri;
            do_fut_with(do_retry(uri), ctx, retry_result_handle);
        } else {
            #[cfg(feature = "log")]
            log_::warn!("重连次数到达上限，终止尝试");
        }
    }
}
