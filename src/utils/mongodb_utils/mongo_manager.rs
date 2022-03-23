use std::collections::HashMap;

use actix_web::web::Data;

use super::{
    db_manager::{DbBuild, DbManager},
    init_mongodb, MongoClient, MongoErr,
};

pub struct MongoManager {
    db_client: MongoClient,
    pub(super) dbs: HashMap<&'static str, DbManager>,
}
pub struct MongoManagerBuild {
    db_client: MongoClient,
    pub(super) dbs: Vec<(&'static str, DbBuild)>,
}

impl Into<MongoManager> for MongoManagerBuild {
    fn into(self) -> MongoManager {
        MongoManager {
            db_client: self.db_client,
            dbs: self.dbs.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl MongoManagerBuild {
    pub fn add_db(&mut self, name: &'static str) {
        let db = self.db_client.database(name.as_ref());
        let db = DbBuild::new(db);
        self.dbs.push((name, db));
    }
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        let db_client = init_mongodb(url.as_ref()).await?;
        Ok(Self {
            db_client,
            dbs: Vec::with_capacity(4),
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
