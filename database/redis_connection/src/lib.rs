#![feature(type_alias_impl_trait)]
mod config;
mod impl_get_connect;
mod impl_initial;
mod static_var;

pub use config::{DbConnectConfig, RedisDbConfig};
pub use database_traits;
pub use impl_get_connect::RedisConnect;
pub use impl_initial::{RedisDatabase, RedisDatabaseBuilder};
pub use redis::RedisError;
