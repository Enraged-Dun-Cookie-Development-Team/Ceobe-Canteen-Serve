use std::any::{type_name, TypeId};

use dashmap::DashMap;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Manager {
    pub db: Database,
    #[builder(default)]
    pub collections: DashMap<TypeId, Collection<()>>,
}

impl Manager {
    pub(crate) fn collection<C, N: Into<Option<&'static str>>>(
        &self, name: N,
    ) -> Collection<C>
    where
        C: Serialize + for<'de> Deserialize<'de>,
        C: 'static,
        C: Sized + Send,
    {
        let name = name.into().unwrap_or(type_name::<C>());
        let id = TypeId::of::<C>();

        let collect = self.db.collection::<C>(name);

        let save = collect.clone_with_type::<()>();
        self.collections.insert(id, save);

        collect
    }
    #[allow(dead_code)]
    pub(crate) fn get_collection<C>(&self) -> Option<Collection<C>>
    where
        C: Serialize + for<'de> Deserialize<'de>,
        C: 'static,
        C: Sized + Send,
    {
        let id = TypeId::of::<C>();
        self.collections
            .get(&id)
            .map(|v| v.value().clone_with_type::<C>())
    }
}
