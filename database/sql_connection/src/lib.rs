mod config;
mod static_vars;

pub use config::{DbConfig, DbConnectConfig, DbOptionsConfig};
pub use sea_orm;
pub use static_vars::{
    connect_to_sql_database, get_sql_database, get_sql_transaction,
};
