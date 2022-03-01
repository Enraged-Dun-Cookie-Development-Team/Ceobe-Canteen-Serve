pub mod traits;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, Statement};

use self::{
    config::{DbConnectConfig, DbOptionsConfig},
    error::{DatabaseError, OrmError},
};

pub mod config;
pub mod error;

#[derive(Debug)]
pub struct ServeDatabase<D>(D);

impl ServeDatabase<sea_orm::DatabaseConnection> {
    pub async fn connect<C>(config: &C) -> Result<Self, DatabaseError>
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

        let mut db_options = ConnectOptions::new(db_url);
        db_options
            .max_connections(config.max_conn())
            .min_connections(config.min_conn())
            .sqlx_logging(config.sql_logger());

        let connect = Database::connect(db_options)
            .await
            .map_err(OrmError::from)?;

        Ok(Self(connect))
    }
}

impl<D> AsRef<D> for ServeDatabase<D>
where
    D: sea_orm::ConnectionTrait,
{
    fn as_ref(&self) -> &D {
        &self.0
    }
}

#[async_trait::async_trait]
impl<D> ConnectionTrait for ServeDatabase<D>
where
    D: ConnectionTrait + Send,
{
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
    }

    async fn execute(&self, stmt: Statement) -> Result<sea_orm::ExecResult, orm_migrate::DbErr> {
        self.0.execute(stmt).await
    }

    async fn query_one(
        &self,
        stmt: Statement,
    ) -> Result<Option<sea_orm::QueryResult>, orm_migrate::DbErr> {
        self.0.query_one(stmt).await
    }

    async fn query_all(
        &self,
        stmt: Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, orm_migrate::DbErr> {
        self.0.query_all(stmt).await
    }
}
