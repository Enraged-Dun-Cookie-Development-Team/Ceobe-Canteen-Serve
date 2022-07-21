use actix_web::web::Data;
use mongo_migration::utils::migrator::MigratorTrait;
use orm_migrate::sql_connection::DbConnectConfig;

use super::{
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

    pub async fn with_config<C: DbConnectConfig>(
        cfg: &C,
    ) -> Result<Self, MongoErr> {
        let url = format!(
            "{}://{}:{}@{}:{}/{}?authSource=admin",
            cfg.scheme(),
            cfg.username(),
            urlencoding::encode(cfg.password()),
            cfg.host(),
            cfg.port(),
            cfg.name()
        );
        Self::new(url).await
    }

    pub async fn collect_migration<M: MigratorTrait + Sync>(
        mut self, migrate: M,
    ) -> Result<Self, mongodb::error::Error> {
        let db = self.inner.get_moved_db();
        let db = migrate.register(db).await?;
        self.inner.set_db(db);
        Ok(self)
    }

    /// 完成构建，生成数据库管理器
    pub fn build(self) -> Data<MongoManager> {
        let v: MongoManager = self.inner.into();
        v.into_data()
    }
}
