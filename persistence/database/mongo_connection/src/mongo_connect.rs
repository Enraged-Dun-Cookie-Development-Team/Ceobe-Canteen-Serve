use mongo_migrate_util::MigratorTrait;
use mongodb::{options::ClientOptions, Database};

use crate::{
    database::builder::DatabaseBuilder, static_vars::set_mongo_database,
    DbConnectConfig, MongoClient, MongoErr,
};

pub struct MongoConnectBuilder {
    db: Option<DatabaseBuilder>,
}

impl MongoConnectBuilder {
    pub async fn new(
        cfg: &impl crate::DbConnectConfig,
    ) -> Result<Self, MongoErr> {
        let client = init_mongodb(format_url(cfg).as_str()).await?;

        let default_db = client.default_database();

        tracing::info!(
            mongodb.database.default =
                Into::<Option<&Database>>::into(&default_db)
                    .map(|db| db.name())
        );

        let db = default_db.map(|db| DatabaseBuilder::new(db, client));
        Ok(Self { db })
    }

    pub async fn apply_mongo_migration<M: MigratorTrait + Sync>(
        mut self, migrate: M,
    ) -> Result<Self, MongoErr> {
        let db = self.db.take().expect("MongoDb数据库未设置");
        let db = migrate.register(db).await?;
        self.db.replace(db);
        Ok(self)
    }

    pub fn build(self) {
        let manage = self.db.expect("MongoDb数据库未设置").into();
        set_mongo_database(manage);
    }
}

async fn init_mongodb(url: &str) -> Result<MongoClient, MongoErr> {
    let mut copts = ClientOptions::parse(url).await?;
    copts.app_name = Some("CeobeCanteen".into());

    let client = MongoClient::with_options(copts)?;
    Ok(client)
}

fn format_url(cfg: &impl DbConnectConfig) -> String {
    let s = format!(
        "{}://{}:{}@{}:{}/{}?authSource=admin",
        cfg.scheme(),
        cfg.username(),
        urlencoding::encode(cfg.password()),
        cfg.host(),
        cfg.port(),
        cfg.name()
    );

    tracing::info!(mongodb.URL = s);
    s
}
