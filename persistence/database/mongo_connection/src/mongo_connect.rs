use mongo_migrate_util::MigratorTrait;
use mongodb::{options::ClientOptions, Database};
use url::Url;

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
    let mut s = Url::parse(&format!(
        "{}://{}:{}@{}:{}/{}",
        cfg.scheme(),
        cfg.username(),
        urlencoding::encode(cfg.password()),
        cfg.host(),
        cfg.port(),
        cfg.name()
    ))
    .expect("MongoDb 连接URL生成异常");

    // 添加查询参数
    for (key, value) in cfg.query() {
        s.query_pairs_mut().append_pair(key, value);
    }

    tracing::info!(mongodb.URL = s.to_string());
    s.to_string()
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[derive(Debug, serde::Deserialize)]
    pub struct MongoDbConfig {
        username: String,
        password: String,
        #[serde(default = "host_default")]
        host: String,
        #[serde(default = "port_default")]
        port: u16,
        db_name: String,
        query: HashMap<String, String>,
    }

    impl DbConnectConfig for MongoDbConfig {
        fn scheme(&self) -> &str { "mongodb" }

        fn username(&self) -> &str { &self.username }

        fn password(&self) -> &str { &self.password }

        fn host(&self) -> &str { &self.host }

        fn port(&self) -> u16 { self.port }

        fn name(&self) -> &str { &self.db_name }

        fn query(&self) -> &HashMap<String, String> { &self.query }
    }

    fn host_default() -> String { "localhost".into() }

    fn port_default() -> u16 { 27017 }

    #[test]
    fn test_format_url() {
        let mut query = HashMap::new();
        query.insert("authSource".to_string(), "admin".to_string());
        query.insert("directConnection".to_string(), "true".to_string());

        let config = MongoDbConfig {
            username: "user".to_string(),
            password: "password".to_string(),
            host: "localhost".to_string(),
            port: 27017,
            db_name: "mydb".to_string(),
            query,
        };

        let expected_url = "mongodb://user:password@localhost:27017/mydb?\
                            authSource=admin&directConnection=true";
        let result = format_url(&config);
        assert_eq!(result, expected_url);
    }
}
