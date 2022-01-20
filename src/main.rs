use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, runtime, sync::broadcast};
use tokio_tungstenite::{
    accept_hdr_async, connect_async,
    tungstenite::{handshake::client::Request, Message},
};
use url::Url;

const DUN_BACKEND: &str = "ws://127.0.0.1/";
const SERVE_URL: &str = "http://localhost";
fn main() {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() {
    // 连接到ws服务器
    let (mut socket, _) = connect_async(Url::parse(DUN_BACKEND).unwrap())
        .await
        .expect("Can not Connect To Ws Server");
    // 广播分发

    let url = Url::parse(SERVE_URL).unwrap();

    let client = Arc::new(
        reqwest::Client::builder()
            .referer(true)
            .build()
            .expect("Create http Client Failure"),
    );

    while let Some(Ok(msg)) = socket.next().await {
        let url = url.clone();
        let lclinet = Arc::clone(&client);
        tokio::spawn(async move {
            if let Message::Text(t) = msg {
                let _ = lclinet.post(url.clone()).body(t).send().await;
            } else if let Message::Binary(b) = msg {
                let _ = lclinet.post(url.clone()).body(b).send().await;
            }
        });
    }
}
