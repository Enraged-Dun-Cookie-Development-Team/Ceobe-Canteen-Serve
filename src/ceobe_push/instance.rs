use std::collections::HashMap;

use crate::Url;
use futures::{future::Inspect, StreamExt};
use tokio::{runtime, sync, io::AsyncWriteExt};
use tokio_tungstenite::connect_async;

use crate::DataItem;

use self::updater::Updater;

mod updater;

type DataCollect = Vec<DataItem>;

const SOURCE_SERVER: &str = "ws://81.68.101.79:5683/";
pub struct Instance {
    updater: updater::Updater,
    // FIX: 中间类型待定
    push_target_reciver: sync::mpsc::Receiver<()>, // push collects
}

impl Instance {
    pub fn new(recive: sync::mpsc::Receiver<()>) -> Self {
        Self {
            updater: Updater::default(),
            push_target_reciver: recive,
        }
    }

    /// 开启单独线程运行
    pub async fn run(mut self) {
        async move {
            // 连接到ws服务器
            let (mut socket, res) = connect_async(Url::parse(SOURCE_SERVER).unwrap())
                .await
                .expect("Can not Connect To Ws Server");


            // TODO:Push Messages
            while let Some(Ok(msg)) = socket.next().await {
                match msg {
                    tokio_tungstenite::tungstenite::Message::Text(t) => {

                        let data=match serde_json::from_str::<HashMap<String,DataCollect>>(&t){
                            Ok(d) => d,
                            Err(e) => {println!("Error from Json {}",e);continue},
                        };
                        let newest=self.updater.check_update(data);

                        println!("{:#?}",newest);
                    },
                    _ => continue,
                }
            }
        }
        .await;
    }
}
