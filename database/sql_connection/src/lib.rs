mod config;
pub mod ext_traits;
mod impl_get_connect;
mod impl_initial;
mod static_vars;

pub use config::{DbConfig, DbConnectConfig, DbOptionsConfig};
pub use database_traits;
pub use impl_get_connect::{SqlConnect, SqlTransaction};
pub use impl_initial::{SqlDatabase, SqlDatabaseBuilder};
pub use sea_orm;

pub type SqlDatabaseOperate =
    database_traits::database_operates::DatabaseOperate<SqlConnect>;

pub use ext_traits::select_only_model::SelectOnlyModel;
pub use sea_orm::EntityTrait;
