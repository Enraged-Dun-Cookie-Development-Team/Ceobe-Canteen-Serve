use crate::Url;
use futures::future::Inspect;
use tokio::{runtime, sync};
use tokio_tungstenite::connect_async;

use crate::DataItem;

use self::updater::Updater;

mod updater;

type DataCollect = Vec<DataItem>;

const SOURCE_SERVER: &str = "ws://127.0.0.1/";
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
    pub async fn run(self) {
        async move {
            // 连接到ws服务器
            let (mut socket, _) = connect_async(Url::parse(SOURCE_SERVER).unwrap())
                .await
                .expect("Can not Connect To Ws Server");

            // TODO:Push Messages
        }.await;
    }
}
