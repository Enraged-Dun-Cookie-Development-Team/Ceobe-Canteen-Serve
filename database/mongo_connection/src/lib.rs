pub mod config;
pub mod static_vars;

pub type MongoDb = mongodb::Database;
pub type MongoClient = mongodb::Client;
pub type MongoErr = mongodb::error::Error;
pub type MongoClientOptions = mongodb::options::ClientOptions;
