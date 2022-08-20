use std::{any::TypeId, collections::HashMap};

use mongo_migrate_util::DbManager;
use mongodb::{Client, Collection};

use crate::MongoDb;

/// 启动时构造数据库信息的类型
/// 可以添加并进行Collection的配置
pub(crate) struct DatabaseBuilder {
    pub(crate) client: Client,
    pub(crate) db: MongoDb,
    pub(crate) inner_collect: HashMap<TypeId, Collection<()>>,
}
impl DatabaseBuilder {
    pub(crate) fn new(db: MongoDb, client: Client) -> Self {
        Self {
            client,
            db,
            inner_collect: HashMap::default(),
        }
    }
}

impl DbManager for DatabaseBuilder {
    fn get_db(&self) -> &mongodb::Database { &self.db }

    fn extent_collections<
        I: IntoIterator<Item = (TypeId, Collection<()>)>,
    >(
        &mut self, iter: I,
    ) {
        self.inner_collect.extend(iter);
    }
}
