use actix_web::web::Data;

use super::{
    db_manager::DbBuild,
    mongo_manager::{MongoManager, MongoManagerBuild},
    MongoErr,
};

pub struct MongoBuild {
    inner: MongoManagerBuild,
}

impl MongoBuild {
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        Ok(Self {
            inner: MongoManagerBuild::new(url).await?,
        })
    }

    pub fn add_db<F>(mut self, name: &'static str, f: F) -> Self
    where
        F: FnOnce(&mut DbBuild),
    {
        self.inner.add_db(name);
        let len = self.inner.dbs.len();
        let (_name, db) = self.inner.dbs.get_mut(len - 1).unwrap();
        f(db);

        self
    }

    pub fn build(self) -> Data<MongoManager> {
        let v: MongoManager = self.inner.into();
        v.into_data()
    }
}
