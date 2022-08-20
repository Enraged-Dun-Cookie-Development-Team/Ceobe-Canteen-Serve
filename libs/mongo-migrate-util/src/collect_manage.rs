use std::{collections::HashSet, ops::Deref};

use mongodb::{options::CreateIndexOptions, Collection, IndexModel};
use tap::Tap;

use crate::MigrationTrait;

pub struct CollectManage<M: MigrationTrait> {
    // inner
    collect: Collection<M::Model>,
    // index set
    idx_set: Option<HashSet<String>>,
}

impl<M: MigrationTrait> Deref for CollectManage<M> {
    type Target = Collection<M::Model>;

    fn deref(&self) -> &Self::Target { &self.collect }
}

impl<M: MigrationTrait> CollectManage<M> {
    pub fn new(collect: Collection<M::Model>) -> Self {
        Self {
            collect,
            idx_set: Default::default(),
        }
    }

    async fn get_idx_set(
        &mut self,
    ) -> Result<&mut HashSet<String>, mongodb::error::Error> {
        let set = self.idx_set.insert(
            self.collect.list_index_names().await?.into_iter().collect(),
        );

        Ok(set)
    }

    pub async fn create_idx_if_not_exist(
        &mut self, index: IndexModel,
        options: impl Into<Option<CreateIndexOptions>>,
    ) -> Result<(), mongodb::error::Error> {
        // get idx set
        let set = match &self.idx_set {
            Some(set) => set,
            None => self.get_idx_set().await?,
        };
        // check idx name is exist
        // if idx name not set ,do not check

        // the name not set
        // the name is not exist
        if index
            .options
            .as_ref()
            .and_then(|opts| opts.name.as_ref())
            // name not set => None : create any way
            // name set = > exist => return true => Some(idx_name) do not
            // create
            //          = > not exist = > return false => None create idx
            .filter(|idx_name| set.contains(*idx_name))
            .is_none()
        {
            // create idx
            let idx_name = self
                .collect
                .create_index(index, options)
                .await?
                .index_name
                .tap(|idx_name| {
                    log::info!(
                        "指针 {:?} 创建完成 {:?}",
                        idx_name,
                        self.collect.name()
                    )
                });

            // add new idx to idx set
            if let Some(set) = &mut self.idx_set {
                set.insert(idx_name);
            }
        }

        Ok(())
    }
}
