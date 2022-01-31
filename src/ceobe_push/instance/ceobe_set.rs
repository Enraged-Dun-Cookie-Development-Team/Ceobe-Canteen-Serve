use std::{collections::HashMap, sync::Arc};

use rresult::IntoSerde;
use tokio::sync;

use crate::ceobe_push::dao::{DataItem, DataSource};

use super::cached::Cached;

pub struct LazyFilter<'s> {
    refer: sync::watch::Ref<'s, HashMap<DataSource, Arc<Cached>>>,
    filter: Vec<String>,
    timestamp: i64,
}

impl<'s, 'b> IntoSerde<'s> for LazyFilter<'b> {
    type Target = HashMap<DataSource, &'s [DataItem]>;

    fn into_serde(&'s self) -> Self::Target {
        self.refer
            .iter()
            .filter(|(k, _v)| {
                if self.filter.len() > 0 {
                    self.filter.contains(&k)
                } else {
                    true
                }
            })
            .map(|(k, v)| (k.clone(), v.new_ceobe_after(self.timestamp)))
            .collect()
    }
}

impl<'s> LazyFilter<'s> {
    pub fn new<'b, S: AsRef<[&'b str]>>(
        refer: sync::watch::Ref<'s, HashMap<DataSource, Arc<Cached>>>,
        filter: &S,
        timestamp: i64,
    ) -> Self {
        Self {
            refer,
            filter: filter.as_ref().iter().map(|s| s.to_string()).collect(),
            timestamp,
        }
    }
}
