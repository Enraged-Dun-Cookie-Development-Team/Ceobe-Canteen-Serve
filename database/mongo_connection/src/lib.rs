mod config;
mod database;
mod error;
mod mongo_connect;
mod static_vars;
mod mongo_utils;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;
pub type MongoClientOptions = mongodb::options::ClientOptions;

pub use config::{DbConnectConfig, MongoDbConfig};
pub use database::manager::DatabaseManage;
pub use error::MongoDbError;
pub use mongo_connect::MongoConnectBuilder;
pub use static_vars::get_mongo_database;
pub use mongo_utils::{manager::Manager, migrator::MigratorTrait, migration::MigrationTrait};
