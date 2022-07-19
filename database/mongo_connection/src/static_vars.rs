use mongodb::Database;
use once_cell::sync::OnceCell;

use crate::{
    config::DbConnectConfig, MongoClient, MongoClientOptions, MongoErr,
};

static MONGO_DATABASE_CONNECTION: OnceCell<Database> = OnceCell::new();

pub async fn connect_to_mongo_database<C>(config: &C) -> Result<(), MongoErr>
where
    C: DbConnectConfig,
{
    let url = format!(
        "{}://{}:{}@{}:{}/{}?authSource=admin",
        config.scheme(),
        config.username(),
        urlencoding::encode(config.password()),
        config.host(),
        config.port(),
        config.name()
    );
    let client_options = MongoClientOptions::parse(url).await?;
    // Get a handle to the deployment.
    let client = MongoClient::with_options(client_options)?;

    let db = client.database("admin");

    if MONGO_DATABASE_CONNECTION.set(db).is_err() {
        panic!("Mongo数据库连接已经建立")
    }
    Ok(())
}

pub fn get_mongo_database() -> &'static Database {
    MONGO_DATABASE_CONNECTION
        .get()
        .expect("Mongo数据库连接未建立")
}
