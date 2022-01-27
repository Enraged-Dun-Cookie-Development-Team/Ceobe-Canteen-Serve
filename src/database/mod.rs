pub mod traits;
use sea_orm::{ConnectOptions, ConnectionTrait, Database};

use self::{
    config::{DbConnetConfig, DbOptionsConfig},
    error::DatabaseError,
};

pub mod config;
pub mod error;

#[derive(Debug)]
pub struct ServeDatabase<D>(D);

impl ServeDatabase<sea_orm::DatabaseConnection> {
    pub async fn connet<C>(config: &C) -> Result<Self, DatabaseError>
    where
        C: DbConnetConfig + DbOptionsConfig,
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

        let mut db_options = ConnectOptions::new(db_url);
        db_options
            .max_connections(config.max_conn())
            .min_connections(config.min_conn())
            .sqlx_logging(config.sql_logger());

        let connet = Database::connect(db_options).await?;

        Ok(Self(connet))
    }
}

impl<'a, D> AsRef<D> for ServeDatabase<D>
where
    D: sea_orm::ConnectionTrait<'a>,
{
    fn as_ref(&self) -> &D {
        &self.0
    }
}

#[async_trait::async_trait]
impl<'a, D> ConnectionTrait<'a> for ServeDatabase<D>
where
    D: ConnectionTrait<'a>,
{
    fn support_returning(&self) -> bool {
        self.as_ref().support_returning()
    }

    fn is_mock_connection(&self) -> bool {
        self.as_ref().is_mock_connection()
    }

    type Stream = D::Stream;

    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.as_ref().get_database_backend()
    }

    async fn execute(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        self.as_ref().execute(stmt).await
    }

    async fn query_one(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
        self.as_ref().query_one(stmt).await
    }

    async fn query_all(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
        self.as_ref().query_all(stmt).await
    }

    fn stream(
        &'a self,
        stmt: sea_orm::Statement,
    ) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Self::Stream, sea_orm::DbErr>> + 'a>>
    {
        self.as_ref().stream(stmt)
    }

    async fn begin(&self) -> Result<sea_orm::DatabaseTransaction, sea_orm::DbErr> {
        self.as_ref().begin().await
    }

    async fn transaction<F, T, E>(&self, callback: F) -> Result<T, sea_orm::TransactionError<E>>
    where
        F: for<'c> FnOnce(
                &'c sea_orm::DatabaseTransaction,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<T, E>> + Send + 'c>,
            > + Send,
        T: Send,
        E: std::error::Error + Send,
    {
        self.as_ref().transaction(callback).await
    }
}
