use once_cell::sync::OnceCell;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, DbErr,
    TransactionTrait,
};

use crate::config::{DbConnectConfig, DbOptionsConfig};

static SQL_DATABASE_CONNECTION: OnceCell<DatabaseConnection> =
    OnceCell::new();

pub async fn connect_to_sql_database<C>(config: &C) -> Result<(), DbErr>
where
    C: DbConnectConfig + DbOptionsConfig,
{
    let db_url = format!(
        "{scheme}://{username}:{password}@{host}:{port}/{name}",
        scheme = config.scheme(),
        username = config.username(),
        password = config.password(),
        host = config.host(),
        port = config.port(),
        name = config.name()
    );

    log::info!("准备连接到数据库: {}", db_url);

    let mut db_options = ConnectOptions::new(db_url);
    if let Some(max_conn) = config.max_conn() {
        db_options.max_connections(max_conn);
    }
    if let Some(min_conn) = config.min_conn() {
        db_options.min_connections(min_conn);
    }

    db_options.sqlx_logging(config.sql_logger());

    let connect = Database::connect(db_options).await?;

    if SQL_DATABASE_CONNECTION.set(connect).is_err() {
        panic!("Sql 数据库连接已经建立")
    }
    Ok(())
}

pub fn get_sql_database() -> &'static DatabaseConnection {
    SQL_DATABASE_CONNECTION.get().expect("Sql 数据库连接未建立")
}

pub async fn get_sql_transaction() -> Result<DatabaseTransaction, DbErr> {
    get_sql_database().begin().await
}
