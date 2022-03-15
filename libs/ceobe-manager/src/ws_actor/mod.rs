mod continuation;
mod heartbeats;
mod income_handle;
mod json_loader;
mod retry_limit;
use std::fmt::Display;
use std::sync::Arc;

use awc::error::WsClientError;

use awc::ClientResponse;

use actix::{io::SinkWrite, Actor, Addr, AsyncContext, Context};
use actix_codec::Framed;

use awc::{ws, BoxedSocket};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use crate::updater_loader::UpdateLoader;

use crate::ws_actor::heartbeats::HeartBeats;
use crate::ws_sender::WsSender;

use self::continuation::Continuation;
use self::heartbeats::BeatTimeout;
use self::json_loader::JsonLoader;
use self::retry_limit::RetryLimit;

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct CeoboWebsocket {
    beat_timeout:BeatTimeout<5>,
    retry: RetryLimit<20>,
    uri: &'static str,
    slink: SinkWrite<ws::Message, WsFramedSink>,
    json_handle: Addr<JsonLoader>,
    continue_handle: Addr<Continuation>,
}

impl CeoboWebsocket {
    pub fn start(
        uri: &'static str,
        sink: WsFramedSink,
        stream: WsFramedStream,
    ) -> (Arc<UpdateLoader>, Arc<WsSender>) {
        #[cfg(feature = "log")]
        log_::info!("Init Ws Actor");

        let (json_handle, updater) = JsonLoader::start();
        let ws = Self::create(|ctx| {
            ctx.add_stream(stream);
            ctx.add_stream(HeartBeats::new());
            Self {
                beat_timeout:Default::default(),
                retry: Default::default(),
                uri,
                slink: SinkWrite::new(sink, ctx),
                json_handle,
                continue_handle: Continuation::start(),
            }
        });

        (UpdateLoader::new(updater), WsSender::new(ws))
    }
}

impl Actor for CeoboWebsocket {
    type Context = Context<CeoboWebsocket>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.slink.close();
        actix::Running::Stop
    }
}

async fn conn_ws<U>(
    uri: U,
) -> Result<(ClientResponse, (WsFramedSink, WsFramedStream)), WsClientError>
where
    actix_http::Uri: TryFrom<U>,
    <actix_http::Uri as TryFrom<U>>::Error: Into<actix_http::error::HttpError>,
    U: Display,
{
    let client = awc::Client::builder().finish();
    #[cfg(feature = "log")]
    log_::info!("开始连接到WebSocket {}", uri);
    let (resp, stream) = client
        .ws(uri)
        .max_frame_size(1024 * 1024 * 2)
        .connect()
        .await?;
    // .expect("无法建立到WebSocket的连接");

    Ok((resp, stream.split()))
}

/// [ws client](https://stackoverflow.com/questions/70118994/build-a-websocket-client-using-actix)
pub async fn start_ws(uri: &'static str) -> (ClientResponse, (Arc<UpdateLoader>, Arc<WsSender>)) {
    let (resp, (sink, stream)) = conn_ws(uri).await.expect("无法建立到WebSocket的连接");
    (resp, CeoboWebsocket::start(uri, sink, stream))
}

#[cfg(test)]
mod test {
    use futures_util::StreamExt;

    use crate::ws_actor::CeoboWebsocket;

    #[test]
    fn test() {
        let system = actix::System::new();

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
            let _addr = CeoboWebsocket::start("ws://81.68.101.79:5683/", slink, stream);

            let _s = actix_rt::signal::ctrl_c().await;
        });
    }
}
