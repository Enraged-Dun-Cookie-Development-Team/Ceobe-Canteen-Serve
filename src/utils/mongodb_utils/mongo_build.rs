use actix_web::web::Data;

use super::{
    db_manager::DbBuild,
    module_register::{self},
    mongo_manager::{MongoManager, MongoManagerBuild},
    MongoErr,
};

/// Mongo 数据库管理构建器
pub struct MongoBuild {
    inner: MongoManagerBuild,
}

impl MongoBuild {
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        Ok(Self {
            inner: MongoManagerBuild::new(url).await?,
        })
    }
    /// 添加一个数据库，并通过 `f` 来配置数据库和内部信息
    pub fn add_db<F>(mut self, name: &'static str, f: F) -> Self
    where
        F: FnOnce(&mut DbBuild),
    {
        let db = self.inner.add_db(name);
        f(db);

        self
    }
    /// 通过数据库注册器注册数据库
    pub fn register_collections<R: module_register::MongoRegister>(self, register: R) -> Self {
        self.add_db(register.db_name(), |db| register.register(db))
    }
    /// 完成构建，生成数据库管理器
    pub fn build(self) -> Data<MongoManager> {
        let v: MongoManager = self.inner.into();
        v.into_data()
    }
}
