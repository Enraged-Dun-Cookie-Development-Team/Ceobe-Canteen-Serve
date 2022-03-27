use actix_web::web::Data;
use futures::Future;

use crate::database::config::DbConnectConfig;

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

    pub async fn with_config<C:DbConnectConfig>(cfg:&C)->Result<Self,MongoErr>{
        let url = format!(
            "{}://{}:{}@{}:{}/{}?authSource=admin",
            cfg.scheme(),cfg.username(),urlencoding::encode(cfg.password()),cfg.host(),cfg.port(),cfg.name()
        );
        Self::new(url).await
    }
    /// 添加一个数据库，并通过 `f` 来配置数据库和内部信息
    pub async fn add_db<F, Fut>(mut self, name: &'static str, f: F) -> Self
    where
        Fut: Future<Output = DbBuild>,
        F: FnOnce(DbBuild) -> Fut,
    {
        self.inner.add_db(name);
        let db = self.inner.dbs.remove(name).unwrap();
        let db = f(db).await;
        self.inner.dbs.insert(name, db);
        self
    }
    /// 通过数据库注册器注册数据库
    pub async fn register_collections<R: module_register::MongoRegister>(
        self,
        register: R,
    ) -> Self {
        self.add_db(register.db_name(), |db| register.register(db))
            .await
    }
    /// 完成构建，生成数据库管理器
    pub fn build(self) -> Data<MongoManager> {
        let v: MongoManager = self.inner.into();
        v.into_data()
    }
}
