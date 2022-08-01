use std::collections::HashSet;

use mongodb::{
    options::CreateIndexOptions, ClientSession, Collection, IndexModel,
};

use crate::MigrationTrait;

pub struct CollectManage<M: MigrationTrait> {
    // inner
    collect: Collection<M::Model>,
    // index set
    idx_set: Option<HashSet<String>>,
}

impl<M: MigrationTrait> CollectManage<M> {
    pub fn new(collect: Collection<M::Model>) -> Self {
        Self {
            collect,
            idx_set: Default::default(),
        }
    }

    async fn get_idx_set(
        &mut self, session: &mut ClientSession,
    ) -> Result<&mut HashSet<String>, mongodb::error::Error> {
        let set = self.idx_set.insert(
            self.collect
                .list_index_names_with_session(session)
                .await?
                .into_iter()
                .collect(),
        );

        Ok(set)
    }

    pub async fn create_idx_if_not_exist(
        &mut self, index: IndexModel,
        options: impl Into<Option<CreateIndexOptions>>,
        session: &mut ClientSession,
    ) -> Result<(), mongodb::error::Error> {
        // get idx set
        let set = match &self.idx_set {
            Some(set) => set,
            None => self.get_idx_set(session).await?,
        };
        // check idx name is exist
        // if idx name not set ,do not check
        if let Some(idx_name) =
            index.options.as_ref().and_then(|opts| opts.name.as_ref())
        {
            if set.contains(idx_name) {
                return Ok(());
            }
        }

        // create idx
        let idx_name = self
            .collect
            .create_index_with_session(index, options, session)
            .await?
            .index_name;

        // add new idx to idx set
        if let Some(set) = &mut self.idx_set {
            set.insert(idx_name);
        }

        Ok(())
    }
}
