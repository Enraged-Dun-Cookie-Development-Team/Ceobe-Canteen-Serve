use actix_web::web::Data;

use super::{
    db_manager::{DbBuild, DbManager},
    init_mongodb, MongoClient, MongoErr,
};

pub struct MongoManager {
    _db_client: MongoClient,
    pub(super) db: DbManager,
}
pub struct MongoManagerBuild {
    db_client: MongoClient,
    pub(super) db: Option<DbBuild>,
}

impl Into<MongoManager> for MongoManagerBuild {
    fn into(self) -> MongoManager {
        MongoManager {
            _db_client: self.db_client,
            db: self.db.expect("默认数据库未找到").into(),
        }
    }
}

impl MongoManagerBuild {
    /// 建立新的 Mongodb 数据库连接
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        let db_client = init_mongodb(url.as_ref()).await?;
        let db = db_client.default_database().map(DbBuild::new);
        Ok(Self { db_client, db })
    }

    pub fn get_moved_db(&mut self) -> DbBuild {
        self.db.take().expect("DbBuild不存在")
    }
    pub fn set_db(&mut self, db: DbBuild) {
        self.db = Some(db)
    }
}

impl MongoManager {
    pub fn get_db(&self) -> &DbManager {
        &self.db
    }

    pub fn into_data(self) -> Data<Self> {
        Data::new(self)
    }
}
