pub mod model_register;
pub mod traits;
use orm_migrate::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, Statement, TransactionTrait,
};
use time_usage::async_time_usage_with_name;

use self::{
    config::{DbConnectConfig, DbOptionsConfig},
    error::DatabaseError
};

pub mod config;
pub mod error;

#[derive(Debug)]
pub struct ServeDatabase<D = sea_orm::DatabaseConnection>(D);

impl ServeDatabase {
    pub async fn register_models(
        self
    ) -> Result<Self, DatabaseError>
    {
        Migrator::up(self.as_ref(), None).await?;
        Ok(self)
    }
}

impl ServeDatabase<sea_orm::DatabaseConnection> {
    pub async fn connect<C>(config: &C) -> Result<Self, DatabaseError>
    where
        C: DbConnectConfig + DbOptionsConfig,
    {
        async_time_usage_with_name("连接到SQL数据库", async {
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
            db_options
                .max_connections(config.max_conn())
                .min_connections(config.min_conn())
                .sqlx_logging(config.sql_logger());

            let connect = Database::connect(db_options).await?;

            Ok(Self(connect))
        })
        .await
    }
}

impl<D> AsRef<D> for ServeDatabase<D>
where
    D: sea_orm::ConnectionTrait,
{
    fn as_ref(&self) -> &D { &self.0 }
}

#[async_trait::async_trait]
impl<D> ConnectionTrait for ServeDatabase<D>
where
    D: ConnectionTrait + Send,
{
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
    }

    async fn execute(
        &self, stmt: Statement,
    ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        async_time_usage_with_name("执行SQL", self.0.execute(stmt)).await
    }

    async fn query_one(
        &self, stmt: Statement,
    ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
        async_time_usage_with_name("查询SQL<1>个", self.0.query_one(stmt))
            .await
    }

    async fn query_all(
        &self, stmt: Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
        async_time_usage_with_name("查询SQL-多个", self.0.query_all(stmt))
            .await
    }
}
#[async_trait::async_trait]
impl<D: TransactionTrait + Sync + Send> TransactionTrait
    for ServeDatabase<D>
{
    async fn begin(
        &self,
    ) -> Result<sea_orm::DatabaseTransaction, sea_orm::DbErr> {
        async_time_usage_with_name("启动SQL事务", self.0.begin()).await
    }

    async fn transaction<F, T, E>(
        &self, callback: F,
    ) -> Result<T, sea_orm::TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c sea_orm::DatabaseTransaction,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<T, E>> + Send + 'c>,
            > + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        async_time_usage_with_name(
            "执行SQL事务",
            self.0.transaction(callback),
        )
        .await
    }
}
