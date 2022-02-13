mod continuation;
use awc::ws::Message;
use std::collections::HashMap;

use actix::{
    fut::wrap_future, io::SinkWrite, Actor, ActorContext, ActorFuture, Addr, AsyncContext, Context,
    StreamHandler,
};
use actix_codec::Framed;

use awc::{error::WsProtocolError, ws, BoxedSocket};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use crate::ceobo_actor::NewCeobeIncome;
use crate::models::{DataItem, DataSource};

use self::continuation::Continuation;

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct CeoboWebsocket {
    slink: SinkWrite<ws::Message, WsFramedSink>,
    updater: Addr<crate::ceobo_actor::Updater>,
    continue_handle: Addr<Continuation>,
}

impl CeoboWebsocket {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> Addr<Self> {
        Self::create(|ctx| {
            ctx.add_stream(stream);
            Self {
                slink: SinkWrite::new(sink, ctx),
                updater: crate::ceobo_actor::Updater::new().start(),
                continue_handle: Continuation::start(),
            }
        })
    }
}

impl actix::io::WriteHandler<awc::error::WsProtocolError> for CeoboWebsocket {}

impl Actor for CeoboWebsocket {
    type Context = Context<CeoboWebsocket>;
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for CeoboWebsocket {
    fn handle(&mut self, item: Result<ws::Frame, WsProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(msg) => {
                match msg {
                    ws::Frame::Text(text) | ws::Frame::Binary(text) => {
                        match serde_json::from_slice::<HashMap<DataSource, Vec<DataItem>>>(&text) {
                            Ok(data) => {
                                let req = self.updater.send(NewCeobeIncome::new_loaded(data));
                                let req_task = async move {
                                    let _resq = req.await.ok();
                                    ()
                                };
                                let actor_task = wrap_future(req_task);
                                ctx.spawn(actor_task);
                            }
                            Err(e) => {
                                self.slink
                                    .write(Message::Text(format!("Wrong Json Format {}", e)));
                            }
                        }
                    }
                    ws::Frame::Continuation(c) => {
                        let req = self.continue_handle.send(continuation::NextIncome(c));
                        let req_task = wrap_future(req);
                        ctx.spawn(req_task.map(|res, a: &mut Self, c| {
                            let task = if let Ok(Some(msg)) = res {
                                match msg {
                                    continuation::FullData::Text(t)
                                    | continuation::FullData::Bin(t) => {
                                        match serde_json::from_slice::<
                                            HashMap<DataSource, Vec<DataItem>>,
                                        >(&t)
                                        {
                                            Ok(data) => {
                                                let req = a
                                                    .updater
                                                    .send(NewCeobeIncome::new_loaded(data));
                                                req
                                            }
                                            Err(e) => {
                                                a.slink.write(Message::Text(format!(
                                                    "Wrong Json Format {}",
                                                    e
                                                )));
                                                let req = a.updater.send(NewCeobeIncome::EMPTY);
                                                req
                                            }
                                        }
                                    }
                                }
                            } else {
                                a.updater.send(NewCeobeIncome::EMPTY)
                            };
                            let wrapped_task = wrap_future(async move {
                                task.await.ok();
                                ()
                            });
                            c.spawn(wrapped_task);
                        }));
                    }
                    ws::Frame::Ping(p) => {
                        println!("Ping!");
                        self.slink.write(Message::Pong(p));
                    }
                    ws::Frame::Pong(_) => println!("Pong!"),
                    ws::Frame::Close(c) => {
                        if let Some(reason) = c {
                            eprintln!(
                                "Websocket Service Close Connection. \ncode :{:?} `{}`",
                                reason.code,
                                reason.description.unwrap_or_default()
                            );
                            ctx.stop()
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Websocket Connect Error: `{}`", err);
                //TODO attempt restart
            }
        }
    }
}

/// [ws client](https://stackoverflow.com/questions/70118994/build-a-websocket-client-using-actix)
async fn wsaa() {
    let cli = actix_web::client::ClientBuilder::new().finish();

    let (a, b) = cli
        .ws("ws://81.68.101.79:5683/")
        .connect()
        .await
        .expect("msg");

    let (slink, stream) = b.split();
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
                .max_frame_size(1024*1024*2)
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
