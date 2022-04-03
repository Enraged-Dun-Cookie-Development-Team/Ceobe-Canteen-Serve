//! mongodb 辅助性独立封装
pub mod config;
pub mod db_manager;
pub mod db_selector;
pub mod error;
pub mod mongo_build;
pub mod mongo_manager;

use mongodb::options::ClientOptions;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;

async fn init_mongodb(url: &str) -> Result<MongoClient, MongoErr> {
    log::info!("连接到Mongodb");
    let mut copts = ClientOptions::parse(url).await?;
    copts.app_name = Some("CeobeCanteen".into());

    let client = MongoClient::with_options(copts)?;
    Ok(client)
}
