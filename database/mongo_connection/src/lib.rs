#![feature(type_alias_impl_trait)]
mod config;
mod database;
mod error;
mod impl_get_connect;
mod impl_initial;
mod mongo_connect;

mod static_vars;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;
pub type MongoClientOptions = mongodb::options::ClientOptions;

pub use config::{DbConnectConfig, MongoDbConfig};
pub use database::manager::{CollectionGuard, DatabaseManage};
pub use database_traits;
pub use error::MongoDbError;
pub use impl_get_connect::MongoConnect;
pub use mongo_connect::MongoConnectBuilder;
pub use static_vars::{get_mongo_collection, get_mongo_database};
