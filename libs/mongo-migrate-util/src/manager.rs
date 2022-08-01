use std::any::TypeId;

use dashmap::{DashMap, DashSet};
use mongodb::{Client, ClientSession, Collection, Database};

use crate::{CollectManage, MigrationTrait};

#[derive(Debug)]
pub struct Manager<'db> {
    pub session: ClientSession,

    pub db: &'db Database,
    pub collections: DashMap<TypeId, Collection<()>>,
    pub name_model_map: DashMap<&'static str, TypeId>,
    pub exist_names: DashSet<String>,
}

impl<'db> Manager<'db> {
    pub async fn new(
        client: &'db Client, db: &'db Database,
    ) -> Result<Manager<'db>, mongodb::error::Error> {
        let mut session = client.start_session(None).await?;
        // get all exist collection name;
        let names = db
            .list_collection_names_with_session(None, &mut session)
            .await?
            .into_iter()
            .collect();
        Ok(Self {
            db,
            collections: DashMap::new(),
            name_model_map: DashMap::new(),
            exist_names: names,
            session,
        })
    }

    /// append an new migration onto the mongodb database
    pub async fn append<M>(
        &mut self, migrate: M,
    ) -> Result<&mut Manager<'db>, mongodb::error::Error>
    where
        M: MigrationTrait,
    {
        // get model type id
        let ty_id = TypeId::of::<M::Model>();
        // using name find type id
        let collection = if let Some(collect_ty) =
            self.name_model_map.get(migrate.name())
        {
            // the collect has been register
            if collect_ty.value() == &ty_id {
                self.collections
                    .get(collect_ty.value())
                    .expect("Collect 注册时异常")
                    .clone_with_type()
            }
            else {
                // same name but diff Model Panic
                panic!("存在同名的collection 但是模型不一致")
            }
        }
        else {
            // collect name not been connect with type ID
            // remove collection from name set, if any
            if self.exist_names.remove(migrate.name()).is_none() {
                // collect not exist
                // create collection
                self.db
                    .create_collection_with_session(
                        migrate.name(),
                        migrate.create_options(),
                        &mut self.session,
                    )
                    .await?;
            }

            // adding to name map
            self.name_model_map.insert(migrate.name(), ty_id);
            // get collection
            let collect = self.db.collection::<M::Model>(migrate.name());
            // adding to  collections
            self.collections.insert(ty_id, collect.clone_with_type());

            collect
        };

        // run migrate
        migrate
            .migrate(CollectManage::new(collection), &mut self.session)
            .await?;

        Ok(self)
    }

    pub async fn apply_all(
        mut self,
    ) -> Result<
        <DashMap<TypeId, Collection<()>> as IntoIterator>::IntoIter,
        mongodb::error::Error,
    > {
        self.session.commit_transaction().await?;
        Ok(self.collections.into_iter())
    }
}
