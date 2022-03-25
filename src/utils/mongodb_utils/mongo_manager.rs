use std::collections::HashMap;

use actix_web::web::Data;

use super::{
    db_manager::{DbBuild, DbManager},
    init_mongodb, MongoClient, MongoErr,
};

pub struct MongoManager {
    _db_client: MongoClient,
    pub(super) dbs: HashMap<&'static str, DbManager>,
}
pub struct MongoManagerBuild {
    db_client: MongoClient,
    pub(super) dbs: HashMap<&'static str, DbBuild>,
}

impl Into<MongoManager> for MongoManagerBuild {
    fn into(self) -> MongoManager {
        MongoManager {
            _db_client: self.db_client,
            dbs: self.dbs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl MongoManagerBuild {
    /// 添加一个数据，并返回创建的数据库的&mut 引用
    /// 如果已经存在同名数据库，将不会创建
    pub fn add_db(&mut self, name: &'static str) -> &mut DbBuild {
        if !self.dbs.contains_key(name) {
            let db = self.db_client.database(name.as_ref());
            let db = DbBuild::new(db);
            self.dbs.insert(name, db);
        }

        self.dbs.get_mut(name).unwrap()
    }
    /// 建立新的 Mongodb 数据库连接
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        let db_client = init_mongodb(url.as_ref()).await?;
        Ok(Self {
            db_client,
            dbs: HashMap::with_capacity(4),
        })
    }
}

impl MongoManager {
    pub fn get_db(&self, name: impl AsRef<str>) -> Option<&DbManager> {
        self.dbs.get(name.as_ref())
    }

    pub fn into_data(self) -> Data<Self> {
        Data::new(self)
    }
}
