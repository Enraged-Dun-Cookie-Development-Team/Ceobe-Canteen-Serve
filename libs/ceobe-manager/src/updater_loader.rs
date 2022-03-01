use std::{
    borrow::Cow,
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
        filter: &[(u64, Cow<'static, str>)],
    ) -> Result<LazyLoad, MailboxError> {
        let mut rec = self.0.lock().unwrap();

        while rec.deref().borrow().is_none() {
            rec.changed().await.ok();
        }

        #[cfg(feature = "log")]
        log_::info!("获取的新饼 源数量{} : {:?}", filter.len(),filter);

        let updated_msg = rec.deref().borrow();
        let mut vec = Vec::with_capacity(16);
        match updated_msg.deref() {
            Some(map) => {
                for (timestamp, (k, v)) in filter.into_iter().filter_map(|(t, ds)| {
                    map.get_key_value(ds.deref().deref())
                        .and_then(|res| Some((*t, res)))
                }) {
                    if let Ok(true) = v.send(CheckCachedUpdate(timestamp)).await {
                        #[cfg(feature = "log")]
                        log_::info!("获取缓存中“{}”数据源最新数据", k.deref());
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
