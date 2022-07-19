use std::{any::TypeId, collections::HashMap};

use mongo_migration::utils::migrator::DbManager;
use mongodb::Collection;

use crate::MongoDb;

/// 启动时构造数据库信息的类型
/// 可以添加并进行Collection的配置
pub(crate) struct DatabaseBuilder {
    pub(crate) db: MongoDb,
    pub(crate) inner_collect: HashMap<TypeId, Collection<()>>,
}
impl DatabaseBuilder {
    pub(crate) fn new(db: MongoDb) -> Self {
        Self {
            db,
            inner_collect: HashMap::default(),
        }
    }
}

impl DbManager for DatabaseBuilder {
    fn get_db(self) -> mongodb::Database { self.db }

    fn from_collects<I: IntoIterator<Item = (TypeId, Collection<()>)>>(
        db: mongodb::Database, iter: I,
    ) -> Self {
        Self {
            db,
            inner_collect: iter.into_iter().collect(),
        }
    }
}
