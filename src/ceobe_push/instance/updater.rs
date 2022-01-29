use std::collections::HashMap;

use chrono::Utc;
use dashmap::DashMap;
use tokio::sync::{self, watch};
use tokio_tungstenite::tungstenite::Message;

use crate::ceobe_push::dao::{DataItem, DataSource};

use super::{cached::Cached, DataCollect};
const DATA_SOURCE_SIZE: usize = 11;

/// Updater 蹲饼器更新器
/// 内部使用`DashMap`保证 Sync+Send
/// # Usage
///
/// ---
///
/// ```rust [no test]
/// // 假设这是最新收到的蹲饼信息
/// let updater = Updater::default();
///
/// let newest: HashMap<String, Vec<DataItem>> = HashMap::default();
/// // new_dun 就是最新的蹲饼信息
/// let new_dun = updater.check_update(newest);
/// ```
///
pub struct Updater {
    recive: sync::mpsc::Receiver<Message>,
    last_id: DashMap<DataSource, Cached>,
}

impl Updater {
    // 预分配空间的构造函数
    pub(crate) fn new() -> (Self, sync::mpsc::Sender<Message>) {
        let (rx, tx) = sync::mpsc::channel(16);
        (
            Self {
                recive: tx,
                last_id: DashMap::with_capacity(DATA_SOURCE_SIZE),
            },
            rx,
        )
    }
    /// 检查更新
    /// 传递最新获取的全部蹲饼数据来源，更新内部缓存信息
    pub(crate) fn check_update(&self, src: HashMap<DataSource, DataCollect>) {
        let now = Utc::now().timestamp();
        src.into_iter()
            .for_each(|(k, v)| self.update_one_source(k, v, now))
    }

    fn update_one_source(&self, src: DataSource, collect: DataCollect, timestamp: i64) {
        if let Some((k, v)) = self.last_id.remove(&src) {
            let cached = v.reflash(collect, timestamp);
            self.last_id.insert(k, cached);
        } else {
            let cached = Cached::new(collect, timestamp);
            self.last_id.insert(src, cached);
        }
    }

    fn into_map(&self) -> HashMap<DataSource, Vec<DataItem>> {
        self.into()
    }

    /// 启动updater 监听，并返接受最新饼的接受端
    pub fn run(mut self) -> sync::watch::Receiver<HashMap<DataSource, Vec<DataItem>>> {
        let (rx, tx) = watch::channel(Default::default());
        tokio::spawn(async move {
            while let Some(msg) = self.recive.recv().await {
                log::info!("Recive From Ws Message");
                match msg {
                    tokio_tungstenite::tungstenite::Message::Text(t) => {
                        let data =
                            match serde_json::from_str::<HashMap<DataSource, DataCollect>>(&t) {
                                Ok(d) => d,
                                Err(e) => {
                                    println!("Error from Json {}", e);
                                    continue;
                                }
                            };
                        self.check_update(data);
                        rx.send(self.into_map()).ok();
                    }
                    _ => continue,
                }
            }
        });
        tx
    }
}

impl<'s> Into<HashMap<DataSource, DataCollect>> for &'s Updater {
    fn into(self) -> HashMap<DataSource, DataCollect> {
        let map = &self.last_id;

        let res = map
            .iter()
            .map(|f| {
                (
                    f.key().clone(),
                    f.value()
                        .into_slice()
                        .into_iter()
                        .map(|f| f.clone())
                        .collect(),
                )
            })
            .collect();
        res
    }
}

#[cfg(test)]
mod test_updater {

    use std::{collections::HashMap, sync::Arc};

    use serde_json::Value;

    use super::Updater;

    fn init_value() -> Value {
        serde_json::json!(
            {
                "Mock":[
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_0",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_1",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_2",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
            ]}
        )
    }

    fn update_value_normal() -> Value {
        serde_json::json!(
            {
                "Mock":[
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_-1",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                    "dataSource":"Mock",
                    "id":"Mock_id_0",
                    "timeForSort":0,
                    "timeForDisplay":"",
                    "content":"Mock",
                    "jumpUrl":"",
                    "imageList":[],
                    "imageHttpList":[]
                 },
                 {
                    "dataSource":"Mock",
                    "id":"Mock_id_1",
                    "timeForSort":0,
                    "timeForDisplay":"",
                    "content":"Mock",
                    "jumpUrl":"",
                    "imageList":[],
                    "imageHttpList":[]
                 },
            ]}
        )
    }

    fn init(init: bool) -> Updater {
        let (updater, _s) = Updater::new();
        if init {
            let mock_init = serde_json::from_value(init_value()).unwrap();
            updater.check_update(mock_init);
        }
        updater
    }

    #[test]
    fn test_no_init() {
        // 第一次启动，没有任何记录
        let updater = init(false);

        assert_eq!(updater.last_id.len(), 0);

        updater.check_update(serde_json::from_value(init_value()).unwrap());
        let res: HashMap<_, _> = (&updater).into();
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(&"Mock".to_string()).unwrap().len(), 3);
        assert_eq!(
            res.get(&"Mock".to_string())
                .and_then(|s| s.get(0).and_then(|d| Some(d.clone())))
                .and_then(|d| Some(d.id)),
            Some(Arc::new("Mock_id_0".to_string()))
        );
    }

    #[test]
    fn test_normal_update() {
        // 第一次更新后
        let updater = init(true);
        assert_eq!(updater.last_id.len(), 1);

        updater.check_update(serde_json::from_value(update_value_normal()).unwrap());
        let res: HashMap<_, _> = (&updater).into();
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(&"Mock".to_string()).unwrap().len(), 3);
        assert_eq!(
            res.get(&"Mock".to_string())
                .and_then(|s| s.get(0).and_then(|d| Some(d.clone())))
                .and_then(|d| Some(d.id)),
            Some(Arc::new("Mock_id_-1".to_string()))
        );
    }
}
