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
    ws_actor::CeoboWebsocket,
};

pub struct UpdateLoader(
    Mutex<watch::Receiver<Option<HashMap<DataSource, Addr<Cached>>>>>,
    Addr<CeoboWebsocket>,
);

impl UpdateLoader {
    pub fn new(rec: UpdaterReceiver, ws: Addr<CeoboWebsocket>) -> Self {
        Self(Mutex::new(rec), ws)
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

        #[cfg(feature = "log")]
        log_::info!("Lazy Load ignores : {:?}", ignores);

        let updated_msg = rec.deref().borrow();
        let mut vec = Vec::with_capacity(16);
        match updated_msg.deref() {
            Some(map) => {
                for (k, v) in map {
                    #[cfg(feature = "log")]
                    log_::info!("Lazy Load Move To : {}", k.as_str());
                    if ignores.contains(&(k.deref().deref())) {
                        continue;
                    }
                    if let Ok(true) = v.send(CheckCachedUpdate(timestamp)).await {
                        let w = v.send(CachedWatcherMsg).await?;
                        let r = v.send(CachedFilter(timestamp)).await?;

                        if !r.is_empty() {
                            vec.push((k.clone(), w, r))
                        }
                    }
                }
            }
            None => unreachable!(),
        }
        #[cfg(feature = "log")]
        log_::info!(
            "Create New LazyLoad time[{}] size[{}]",
            timestamp,
            vec.len()
        );
        Ok(LazyLoad(vec))
    }
}

pub struct LazyLoad(pub(crate) Vec<(AShareString, watch::Receiver<Vec<DataItem>>, Range<usize>)>);

impl LazyLoad {
    pub fn into_not_empty(self) -> Option<Self> {
        #[cfg(feature = "log")]
        log_::info!("Checking LazyLoad Is Empty size[{}]", self.0.len());

        if self.0.len() == 0 {
            None
        } else {
            Some(self)
        }
    }
}

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
