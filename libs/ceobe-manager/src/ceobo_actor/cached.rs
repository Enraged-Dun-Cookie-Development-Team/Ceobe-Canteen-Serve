use std::sync::atomic::{AtomicU64, Ordering};

use actix::{Actor, Context, Handler, MessageResult};
use dashmap::DashMap;
use tokio::sync::watch;

use crate::models::{CachedId, DataItem};

use super::{CachedFilter, CachedUpdateMsg, CachedWatcherMsg, CheckCachedUpdate};

#[derive(Debug)]
pub struct Cached {
    /// 记录的上次更新时间
    update_time: AtomicU64,
    /// 记录的每条记录上次更新时间
    last_record_time: DashMap<CachedId, u64>,
    /// 记录的上次的缓存的数据
    /// 发送给接送端的使用
    sender: watch::Sender<Vec<DataItem>>,
    recv: watch::Receiver<Vec<DataItem>>,
}

impl Cached {
    pub fn new(timestamp: u64, data: Vec<DataItem>) -> Self {
        #[cfg(feature = "log")]
        {
            log_::info!("New DataSource Cached Created at TimeStamp({})", timestamp);
        }

        let update_time = AtomicU64::new(timestamp);
        let last_record_time = data.iter().map(|f| (f.id.clone(), timestamp)).collect();
        let (sender, recv) = watch::channel(data);

        Self {
            update_time,
            last_record_time,
            sender,
            recv,
        }
    }
}

impl Actor for Cached {
    type Context = Context<Self>;
}

impl Handler<CachedUpdateMsg> for Cached {
    type Result = MessageResult<CachedUpdateMsg>;

    fn handle(&mut self, msg: CachedUpdateMsg, _ctx: &mut Self::Context) -> Self::Result {
        let CachedUpdateMsg {
            res_timestamp,
            data,
            ..
        } = msg;

        #[cfg(feature = "log")]
        {
            log_::info!(
                "DataSource Cached Updating at TimeStamp({}) Size:[{}]",
                res_timestamp,
                data.len()
            );
        }

        self.update_time.store(res_timestamp, Ordering::Release);

        // let mut records = Vec::with_capacity(data.len());

        let (map, records): (DashMap<_, _>, Vec<_>) = data
            .into_iter()
            .map(|d| (d.id.clone(), d))
            .filter_map(|(id, d)| {
                if let Some((k, v)) = self.last_record_time.remove(&id) {
                    Some(((k, v), d))
                } else {
                    Some(((id, res_timestamp), d))
                }
            })
            .unzip();
        self.last_record_time.clear();
        self.last_record_time.extend(map);
        self.sender.send(records).ok();
        MessageResult(())
    }
}

impl Handler<CheckCachedUpdate> for Cached {
    type Result = MessageResult<CheckCachedUpdate>;

    fn handle(&mut self, msg: CheckCachedUpdate, _ctx: &mut Self::Context) -> Self::Result {
        let time = msg.0;
        let res = if time > self.update_time.load(Ordering::Relaxed) {
            false
        } else {
            self.last_record_time.iter().any(|f| f.value() > &time)
        };
        #[cfg(feature = "log")]
        {
            log_::info!("DataSource Cached Updated Check {}", res);
        }
        MessageResult(res)
    }
}

impl Handler<CachedWatcherMsg> for Cached {
    type Result = MessageResult<CachedWatcherMsg>;

    fn handle(&mut self, _msg: CachedWatcherMsg, _ctx: &mut Self::Context) -> Self::Result {
        #[cfg(feature = "log")]
        {
            log_::info!("Prevent Cached Watcher");
        }
        MessageResult(self.sender.subscribe())
    }
}

impl Handler<CachedFilter> for Cached {
    type Result = MessageResult<CachedFilter>;

    fn handle(&mut self, msg: CachedFilter, _ctx: &mut Self::Context) -> Self::Result {
        let time = msg.0;

        let start_idx = self
            .recv
            .borrow()
            .iter()
            .map(|k| *self.last_record_time.get(&k.id).unwrap())
            .enumerate()
            .find(|(_u, v)| v > &time)
            .and_then(|f| Some(f.0))
            .unwrap_or_default();
        let range = start_idx..self.recv.borrow().len();
        #[cfg(feature = "log")]
        {
            log_::info!("Loading New Cached After {} range {:?}", time, range);
        }
        MessageResult(range)
    }
}
