use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    sync::Arc,
};

use mongodb::{options::CollectionOptions, Collection};
use serde::{Deserialize, Serialize};

use super::MongoDb;

pub struct DbBuild {
    db: MongoDb,
    inner_collect: HashMap<TypeId, Collection<()>>,
}
pub struct DbManager {
    inner_collect: Arc<HashMap<TypeId, Collection<()>>>,
}

impl Clone for DbManager {
    fn clone(&self) -> Self {
        Self {
            inner_collect: self.inner_collect.clone(),
        }
    }
}

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

        let data = data.clone_with_type::<()>();
        self.inner_collect.insert(id, data);
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
            inner_collect: Arc::new(self.inner_collect),
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

        let collect = self.inner_collect.get(&id)?.clone_with_type::<C>();

        Some(collect)
    }
}
