use mongo_migration::utils::migrator::MigratorTrait;
use mongodb::{options::ClientOptions, Database};

use crate::{
    database::builder::DatabaseBuilder, static_vars::set_mongo_database,
    MongoClient, MongoErr,
};

pub struct MongoConnectBuilder {
    _db_client: MongoClient,
    db: Option<DatabaseBuilder>,
}

impl MongoConnectBuilder {
    pub async fn new(url: impl AsRef<str>) -> Result<Self, MongoErr> {
        let client = init_mongodb(url.as_ref()).await?;

        let default_db = client.default_database();
        log::info!(
            "默认数据库为 {:?}",
            Into::<Option<&Database>>::into(&default_db).map(|db| db.name())
        );

        let db = default_db.map(DatabaseBuilder::new);
        Ok(Self {
            _db_client: client,
            db,
        })
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
    log::info!("连接到Mongodb");
    let mut copts = ClientOptions::parse(url).await?;
    copts.app_name = Some("CeobeCanteen".into());

    let client = MongoClient::with_options(copts)?;
    Ok(client)
}
