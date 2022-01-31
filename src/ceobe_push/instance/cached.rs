use dashmap::DashMap;
use std::ops::Deref;

use crate::{ceobe_push::dao::CachedId, DataItem};

/// 饼信息缓存器，一个实例用于缓存一个DataSource 的饼信息
pub struct Cached {
    reflesh_time: i64,
    // 缓存
    pub(super) cached: Vec<DataItem>,
    // map
    map: DashMap<CachedId, i64>,
}

impl Cached {
    pub fn new(cached: Vec<DataItem>, timestamp: i64) -> Self {
        let (map, cached): (DashMap<_, _>, Vec<_>) = cached
            .into_iter()
            .map(|d| (d.id.clone(), d))
            .map(|(id, v)| ((id, timestamp), v))
            .unzip();

        Self {
            cached,
            map,
            reflesh_time: timestamp,
        }
    }

    pub fn reflash(self, newest: Vec<DataItem>, timestamp: i64) -> Self {
        let (map, cached): (DashMap<_, _>, Vec<_>) = newest
            .into_iter()
            // 拆分 id 和数据
            .map(|d| (d.id.clone(), d))
            .map(|(id, v)| {
                // 如果数据存在原先的缓存中，使用原来缓存的时间戳
                if let Some((id, ts)) = self.map.remove(&id) {
                    ((id, ts), v)
                }
                // 否则使用外部给定的新的时间戳
                else {
                    ((id, timestamp), v)
                }
            })
            .unzip();
        Self{
            cached,
            map,
            reflesh_time: timestamp,
        }
    }

    pub fn into_slice(&self) -> &[DataItem] {
        self.cached.as_slice()
    }
}

impl Cached {
    pub fn new_ceobe_after(&self, timestamp: i64) -> &[DataItem] {
        if self.cached.len() > 0 {
            let mut idx = 0;
            self.cached.iter().for_each(|v| {
                if let Some(v) = self.map.get(&v.id) {
                    if v.deref() > &timestamp {
                        idx += 1;
                    }
                }
            });
            if idx > 0 {
                &self.cached[..idx]
            } else {
                &[]
            }
        } else {
            &[]
        }
    }

    pub fn is_refleshed(&self, timestamp: i64) -> bool {
        self.reflesh_time > timestamp
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::ceobe_push::dao::DataItem;

    use super::Cached;

    #[test]
    fn test_reflesh() {
        let mut t: DataItem = Default::default();
        t.id = Arc::new(String::from("Mock 1"));
        let cached = Cached::new(vec![t], 1);

        let newest = cached.new_ceobe_after(0);

        assert_eq!(newest.len(), 1);

        let mut t1: DataItem = Default::default();
        t1.id = Arc::new(String::from("Mock 1"));
        let mut t2: DataItem = Default::default();
        t2.id = Arc::new(String::from("Mock 2"));

        let new = cached.reflash(vec![t1, t2], 2);

        let newest = new.new_ceobe_after(0);

        assert_eq!(newest.len(), 2);

        let newest = new.new_ceobe_after(1);
        assert_eq!(newest.len(), 1);

        let newest = new.new_ceobe_after(2);
        assert_eq!(newest.len(), 0);

        assert!(!new.is_refleshed(3));
        assert!(new.is_refleshed(0));
    }
}
