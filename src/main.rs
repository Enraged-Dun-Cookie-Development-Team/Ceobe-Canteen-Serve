use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, runtime, sync::broadcast};
use tokio_tungstenite::{
    accept_hdr_async, connect_async,
    tungstenite::{handshake::client::Request, Message},
};
use url::Url;



const DUN_BACKEND: &str = "ws://127.0.0.1/";
const SERVE_URL: &str = "127.0.0.1:9001";
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
    let (rx, _) = broadcast::channel::<Message>(128);

    let revi = rx.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = socket.next().await {
            let _ = revi.send(msg);
        }
        socket.close(None).await
    });

    // Tcp Server
    let listener = TcpListener::bind(SERVE_URL)
        .await
        .expect(&format!("Can Not Bind To {}", SERVE_URL));

    //listening
    while let Ok((stream, _)) = listener.accept().await {
        let mut rec = rx.subscribe();
        let peer_addr = stream.peer_addr().expect("Connect should have Peer Addr");
        tokio::spawn(async move {
            let mut ws_stream = accept_hdr_async(stream, |req: &Request, res| {
                let uri = req.uri();
                println!("Accept URI : {}", uri);
                Ok(res)
            })
            .await
            .expect("Faliure Accept Ws");

            println!("Connet To {}", peer_addr);
            while let Ok(msg) = rec.recv().await {
                let _ = ws_stream.send(msg).await;
            }

            ws_stream.close(None).await
        });
    }
}
