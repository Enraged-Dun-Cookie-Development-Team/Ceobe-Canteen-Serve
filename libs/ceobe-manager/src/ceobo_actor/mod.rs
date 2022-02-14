mod cached;
mod updater;

use std::collections::HashMap;

use crate::models::{DataItem, DataSource};

use std::ops::Range;

pub use cached::Cached;
pub use updater::Updater;
pub use updater::UpdaterReceiver;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct CachedUpdateMsg {
    res_timestamp: u64,
    data: Vec<DataItem>,
}

#[derive(actix::Message)]
#[rtype(result = "tokio::sync::watch::Receiver<Vec<DataItem>>")]
pub struct CachedWatcherMsg;

#[derive(actix::Message)]
#[rtype(result = "bool")]
pub struct CheckCachedUpdate(pub(crate) u64);

#[derive(actix::Message)]
#[rtype(result = "Range<usize>")]
pub struct CachedFilter(pub(crate) u64);

#[derive(actix::Message)]
#[rtype(result = "()")]
pub enum NewCeobeIncome {
    Loaded(HashMap<DataSource, Vec<DataItem>>),
}

impl NewCeobeIncome {
    pub fn new_loaded(map: HashMap<DataSource, Vec<DataItem>>) -> Self {
        Self::Loaded(map)
    }
}
