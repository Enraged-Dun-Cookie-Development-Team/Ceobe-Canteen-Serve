pub use config::{DbConnectConfig, MongoDbConfig};
pub use database::manager::{CollectionGuard, DatabaseManage};
pub use database_traits;
pub use error::MongoDbError;
pub use impl_get_connect::{MongoConnect, MongoDbCollectionTrait};
pub use mongo_connect::MongoConnectBuilder;

mod config;
mod database;
mod error;
mod impl_get_connect;
mod impl_initial;
mod mongo_connect;

mod static_vars;
pub mod utils;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;
pub type MongoClientOptions = mongodb::options::ClientOptions;

pub type MongoDatabaseOperate =
    database_traits::database_operates::DatabaseOperate<MongoConnect>;
