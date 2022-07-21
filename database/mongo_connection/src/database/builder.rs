use std::{any::TypeId, collections::HashMap};

use mongo_migrate_util::DbManager;
use mongodb::{Client, Collection};

use crate::MongoDb;

/// 启动时构造数据库信息的类型
/// 可以添加并进行Collection的配置
pub(crate) struct DatabaseBuilder {
    pub(crate) client: Client,
    pub(crate) db: Option<MongoDb>,
    pub(crate) inner_collect: HashMap<TypeId, Collection<()>>,
}
impl DatabaseBuilder {
    pub(crate) fn new(db: MongoDb, client: Client) -> Self {
        Self {
            db: db.into(),
            inner_collect: HashMap::default(),
            client,
        }
    }
}

impl DbManager for DatabaseBuilder {
    fn get_db(&mut self) -> mongodb::Database {
        self.db.take().expect("Mongo Database 正在被占用")
    }

    fn extent_collections<
        I: IntoIterator<Item = (TypeId, Collection<()>)>,
    >(
        &mut self, db: mongodb::Database, iter: I,
    ) {
        self.db.replace(db);
        self.inner_collect.extend(iter);
    }
}
