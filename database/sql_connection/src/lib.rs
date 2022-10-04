#![feature(type_alias_impl_trait)]
mod config;
mod impl_get_connect;
mod impl_initial;
mod static_vars;

pub use config::{DbConfig, DbConnectConfig, DbOptionsConfig};
pub use database_traits;
pub use impl_get_connect::{
    SqlConnect, SqlDatabaseConnectTrait, SqlDatabaseTransactionTrait,
    SqlTransaction,
};
pub use impl_initial::{SqlDatabase, SqlDatabaseBuilder};
pub use sea_orm;
pub use static_vars::{
    connect_to_sql_database, get_sql_database, get_sql_transaction,
};
