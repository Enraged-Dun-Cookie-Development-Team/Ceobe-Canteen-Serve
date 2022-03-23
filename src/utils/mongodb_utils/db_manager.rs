use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
};

use mongodb::{options::CollectionOptions, Collection};
use serde::{Deserialize, Serialize};

use super::MongoDb;

pub struct DbBuild {
    db: MongoDb,
    inner_collect: HashMap<TypeId, Box<dyn std::any::Any>>,
}
pub struct DbManager {
    inner_collect: HashMap<TypeId, Box<dyn std::any::Any>>,
}

unsafe impl Sync for DbManager {}
unsafe impl Send for DbManager {}

impl DbBuild {
    pub fn add_collection_option<C>(&mut self, opt: Option<CollectionOptions>)
    where
        C: for<'de> Deserialize<'de> + Serialize,
        C: 'static,
        C: Sized,
        C: Send,
    {
        let id = std::any::TypeId::of::<C>();
        let name = type_name::<C>();
        // insert
        let data = self
            .db
            .collection_with_options::<C>(name, opt.unwrap_or_default());
        let boxed = Box::new(data) as Box<dyn Any>;
        self.inner_collect.insert(id, boxed);
    }
    pub fn add_collection<C>(&mut self)
    where
        C: for<'de> Deserialize<'de> + Serialize,
        C: 'static,
        C: Sized,
        C: Send,
    {
        self.add_collection_option::<C>(None);
    }
    pub fn new(db: MongoDb) -> Self {
        Self {
            db,
            inner_collect: HashMap::default(),
        }
    }
}

impl Into<DbManager> for DbBuild {
    fn into(self) -> DbManager {
        DbManager {
            inner_collect: self.inner_collect,
        }
    }
}

impl DbManager {
    pub fn collection<C>(&self) -> Option<Collection<C>>
    where
        C: for<'de> Deserialize<'de> + Serialize,
        C: 'static,
        C: Sized,
    {
        let id = std::any::TypeId::of::<C>();

        let collect = self
            .inner_collect
            .get(&id)?
            .downcast_ref::<Collection<C>>()?
            .clone();

        Some(collect)
    }
}
