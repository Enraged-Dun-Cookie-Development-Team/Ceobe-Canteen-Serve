use sea_orm::{ConnectOptions, Database};

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

