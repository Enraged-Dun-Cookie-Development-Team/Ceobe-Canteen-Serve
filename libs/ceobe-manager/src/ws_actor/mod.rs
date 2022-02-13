use actix::{io::SinkWrite, Actor, Addr, AsyncContext, Context, StreamHandler};
use actix_codec::Framed;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use awc::error::WsProtocolError;
use awc::{ws, BoxedSocket};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct CeoboWebsocket {
    slink: SinkWrite<ws::Message, WsFramedSink>,
}

impl CeoboWebsocket {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> Addr<Self> {
        Self::create(|ctx| {
            ctx.add_stream(stream);
            Self {
                slink: SinkWrite::new(sink, ctx),
            }
        })
    }
}

impl actix::io::WriteHandler<awc::error::WsProtocolError> for CeoboWebsocket {}

impl Actor for CeoboWebsocket {
    type Context = Context<CeoboWebsocket>;
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for CeoboWebsocket {
    fn handle(&mut self, item: Result<ws::Frame, WsProtocolError>, ctx: &mut Self::Context) {}
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
