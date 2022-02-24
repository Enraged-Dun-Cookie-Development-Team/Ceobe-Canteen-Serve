mod continuation;
mod json_loader;
use awc::ws::Message;
use awc::ClientResponse;

use actix::{io::SinkWrite, Actor, ActorContext, Addr, AsyncContext, Context, StreamHandler};
use actix_codec::Framed;

use awc::{error::WsProtocolError, ws, BoxedSocket};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use crate::fut_utils::{do_fut, do_fut_with};
use crate::updater_loader::UpdateLoader;

use self::continuation::Continuation;
use self::json_loader::JsonLoader;

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct CeoboWebsocket {
    slink: SinkWrite<ws::Message, WsFramedSink>,
    json_handle: Addr<JsonLoader>,
    continue_handle: Addr<Continuation>,
}

impl CeoboWebsocket {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> UpdateLoader {
        #[cfg(feature = "log")]
        log_::info!("Init Ws Actor");

        let (json_handle, updater) = JsonLoader::start();
        UpdateLoader::new(
            updater,
            Self::create(|ctx| {
                ctx.add_stream(stream);
                Self {
                    slink: SinkWrite::new(sink, ctx),
                    json_handle,
                    continue_handle: Continuation::start(),
                }
            }),
        )
    }
}

impl actix::io::WriteHandler<awc::error::WsProtocolError> for CeoboWebsocket {}

impl Actor for CeoboWebsocket {
    type Context = Context<CeoboWebsocket>;
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for CeoboWebsocket {
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
                    log_::info!("Ping!");
                    self.slink.write(Message::Pong(p));
                }
                ws::Frame::Pong(_) => {
                    #[cfg(feature = "log")]
                    log_::info!("Pong!")
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
}

/// [ws client](https://stackoverflow.com/questions/70118994/build-a-websocket-client-using-actix)
pub async fn start_ws(uri: &str) -> (ClientResponse, UpdateLoader) {
    let client = awc::Client::builder().finish();
    #[cfg(feature = "log")]
    log_::info!("Connect Ws Client To {}", uri);
    let (resp, stream) = client
        .ws(uri)
        .max_frame_size(1024 * 1024 * 2)
        .connect()
        .await
        .expect("connect Failure");

    let (sink, stream) = stream.split();
    
    (resp, CeoboWebsocket::start(sink, stream))
}

#[cfg(test)]
mod test {
    use futures_util::StreamExt;

    use crate::ws_actor::CeoboWebsocket;

    #[test]
    fn test() {
        let mut system = actix::System::new("test");

        system.block_on(async {
            let client = awc::Client::builder().finish();
            let (mut resp, stream) = client
                .ws("ws://81.68.101.79:5683/")
                .max_frame_size(1024 * 1024 * 2)
                .connect()
                .await
                .unwrap();
            let bod = resp.body().await;
            println!("{:?}", bod);

            let (slink, stream) = stream.split();

            let _addr = CeoboWebsocket::start(slink, stream);

            let _s = actix_rt::signal::ctrl_c().await;
        });
    }
}
