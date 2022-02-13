mod updater;
mod cached;
use std::{sync::atomic::AtomicU64, collections::HashMap};

use crate::models::{DataItem, DataSource};
use dashmap::DashMap;
use std::ops::Range;

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
pub struct CheckCachedUpdate(u64);

#[derive(actix::Message)]
#[rtype(result = "Range<usize>")]
pub struct CachedFilter(u64);

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct NewCeobeIncome(HashMap<DataSource,Vec<DataItem>>);