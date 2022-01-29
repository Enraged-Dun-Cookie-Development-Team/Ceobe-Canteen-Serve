use crate::Url;
use futures::StreamExt;
use tokio::sync;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::DataItem;

use self::updater::Updater;

pub mod cached;
pub mod ceobe_set;
mod updater;

pub type DataCollect = Vec<DataItem>;

const SOURCE_SERVER: &str = "ws://81.68.101.79:5683/";
pub struct Instance {
    updater: sync::mpsc::Sender<Message>,
}

impl Instance {
    pub fn new() -> (Self, Updater) {
        let (u, s) = Updater::new();
        (Self { updater: s }, u)
    }

    /// 开启单独线程运行
    pub async fn run(self) {
        async move {
            // 连接到ws服务器
            let (mut socket, _res) = connect_async(Url::parse(SOURCE_SERVER).unwrap())
                .await
                .expect("Can not Connect To Ws Server");

            while let Some(Ok(msg)) = socket.next().await {
                match self.updater.send(msg).await {
                    Ok(_ok) => (),
                    Err(er) => {
                        log::error!("can not Send Msg {}", er);
                        continue;
                    }
                };
            }
        }
        .await;
    }
}
