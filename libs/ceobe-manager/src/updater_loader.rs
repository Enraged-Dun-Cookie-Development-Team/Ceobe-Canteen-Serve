use std::{
    collections::HashMap,
    ops::{Deref, Index, Range},
    sync::Mutex,
};

use actix::{Addr, MailboxError};
use serde::{ser::SerializeMap, Serialize};
use tokio::sync::watch;

use crate::{
    ceobo_actor::{Cached, CachedFilter, CachedWatcherMsg, CheckCachedUpdate, UpdaterReceiver},
    models::{AShareString, DataItem, DataSource},
};

pub struct UpdateLoader(Mutex<watch::Receiver<Option<HashMap<DataSource, Addr<Cached>>>>>);

impl UpdateLoader {
    pub fn new(rec: UpdaterReceiver) -> Self {
        Self(Mutex::new(rec))
    }
}

impl UpdateLoader {
    pub async fn lazy_load(
        &self,
        timestamp: u64,
        ignores: &[&str],
    ) -> Result<LazyLoad, MailboxError> {
        let mut rec = self.0.lock().unwrap();

        while rec.deref().borrow().is_none() {
            rec.changed().await.ok();
        }

        let updated_msg = rec.deref().borrow();
        let mut vec = Vec::with_capacity(16);
        match updated_msg.deref() {
            Some(map) => {
                for (k, v) in map {
                    if ignores.contains(&(k.deref().deref())) {
                        continue;
                    }
                    if let Ok(true) = v.send(CheckCachedUpdate(timestamp)).await {
                        let w = v.send(CachedWatcherMsg).await?;
                        let r = v.send(CachedFilter(timestamp)).await?;

                        vec.push((k.clone(), w, r))
                    }
                }
            }
            None => unreachable!(),
        }

        Ok(LazyLoad(vec))
    }
}

pub struct LazyLoad(pub(crate) Vec<(AShareString, watch::Receiver<Vec<DataItem>>, Range<usize>)>);

impl Serialize for LazyLoad {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, vf, fr) in &self.0 {
            let ref_val = vf.borrow();
            let ran = ref_val.index(fr.clone());
            map.serialize_entry(k, ran)?;
        }
        map.end()
    }
}
