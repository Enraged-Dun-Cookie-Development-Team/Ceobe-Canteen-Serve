use std::ops::Deref;
use std::collections::HashMap;

use lazy_static::__Deref;
use rresult::IntoSerde;
use tokio::sync;

use crate::ceobe_push::dao::{DataSource, DataItem};

use super::cached::Cached;


pub struct LazyFilter<'s>{
    refer:sync::watch::Ref<'s,HashMap<DataSource,Cached>>,
    filter:&'s [String],
    timestamp:i64
}

impl<'s> IntoSerde<'s> for LazyFilter<'s> {
    type Target=HashMap<DataSource,&'s [DataItem]>;

    fn into_serde(&'s self) -> Self::Target {
        self.refer.iter()
        .filter(|(k,_v)|self.filter.contains(&k))
        .map(|(k,v)|(k.clone(),v.new_ceobe_after(self.timestamp)))
        .collect()
    }
}

