use std::any::{type_name, TypeId};

use dashmap::{DashMap, DashSet};
use mongodb::{Collection, Database};
use tap::Tap;

use crate::{CollectManage, MigrationTrait};

#[derive(Debug)]
pub struct Manager<'db> {
    pub db: &'db Database,
    pub collections: DashMap<TypeId, Collection<()>>,
    pub name_model_map: DashMap<&'static str, TypeId>,
    pub exist_names: DashSet<String>,
}

impl<'db> Manager<'db> {
    pub async fn new(
        db: &'db Database,
    ) -> Result<Manager<'db>, mongodb::error::Error> {
        // get all exist collection name;
        let names = db
            .list_collection_names(None)
            .await?
            .tap(|vec| log::info!("当前MongoDb数据库内有Collect : {vec:?}"))
            .into_iter()
            .collect();
        Ok(Self {
            db,
            collections: DashMap::new(),
            name_model_map: DashMap::new(),
            exist_names: names,
        })
    }

    /// append an new migration onto the mongodb database
    pub async fn append<M>(
        &mut self, migrate: M,
    ) -> Result<&mut Manager<'db>, mongodb::error::Error>
    where
        M: MigrationTrait,
    {
        log::debug!(
            "Migrate Mongo 数据库, name : {:?}, model : `{}`",
            migrate.name(),
            type_name::<M::Model>()
        );
        // get model type id
        let ty_id = TypeId::of::<M::Model>();

        // using name find type id
        let collection = if let Some(collect_ty) =
            self.name_model_map.get(migrate.name())
        {
            log::debug!(
                "该 collection 已经被注册, 从已有表中提取信息 {:?}",
                migrate.name()
            );
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
            log::debug!(
                "该 collection 还未被注册 检查是否在 已有的collect中 {:?}",
                migrate.name()
            );
            // collect name not been connect with type ID
            // remove collection from name set, if any
            if self.exist_names.remove(migrate.name()).is_none() {
                log::debug!(
                    "该 collection 未被创建, 创建colletion {:?}",
                    migrate.name()
                );
                // collect not exist
                // create collection
                self.db
                    .create_collection(
                        migrate.name(),
                        migrate.create_options(),
                    )
                    .await?;
            }

            log::debug!("注册 collection {:?}", migrate.name());
            // adding to name map
            self.name_model_map.insert(migrate.name(), ty_id);
            // get collection
            let collect = self.db.collection::<M::Model>(migrate.name());
            // adding to  collections
            self.collections.insert(ty_id, collect.clone_with_type());

            collect
        };

        // run migrate
        migrate.migrate(CollectManage::new(collection)).await?;

        Ok(self)
    }

    pub async fn done(
        self,
    ) -> Result<
        <DashMap<TypeId, Collection<()>> as IntoIterator>::IntoIter,
        mongodb::error::Error,
    > {
        Ok(self.collections.into_iter())
    }
}
