#![feature(type_alias_impl_trait)]
mod impl_initial;
mod static_var;
mod config;
mod impl_get_connect;

pub use config::{RedisDbConfig, DbConnectConfig};
pub use database_traits;
pub use impl_get_connect::{RedisConnect};
pub use impl_initial::{RedisDatabase, RedisDatabaseBuilder};
pub use redis::RedisError;