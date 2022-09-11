#![feature(generic_associated_types)]

use std::{error::Error as StdError, future::Future};

pub trait DatabaseInitialBasic {
    type Error: StdError;
    /// start up builder
    type Builder;
}

pub trait DatabaseInitialConnect<Params: 'static>:
    DatabaseInitialBasic
{
    /// future returning
    type ConnectFuture<'p>: Future<Output = Result<Self::Builder, Self::Error>>
        + 'p;

    /// connect to the database
    fn start_connect(params: &Params) -> Self::ConnectFuture<'_>;
}

pub trait DatabaseInitialMigration<Params: 'static>:
    DatabaseInitialBasic
{
    type MigrateFuture<'s>: Future<Output = Result<Self::Builder, Self::Error>>
        + 's;

    /// apply the database migration
    fn apply_migration(
        builder: Self::Builder, params: &Params,
    ) -> Self::MigrateFuture<'_>;
}

/// a trait that for database init itself
pub trait DatabaseInitial: DatabaseInitialBasic + Sized {
    type BuildResult;

    fn build(
        builder: Self::Builder,
    ) -> Result<Self::BuildResult, Self::Error>;
}

pub async fn connect_db<D, C>(
    connect_params: &C,
) -> Result<D::BuildResult, D::Error>
where
    D: DatabaseInitialConnect<C> + DatabaseInitial,
    C: 'static,
{
    let builder = D::start_connect(connect_params).await?;

    D::build(builder)
}

pub async fn connect_db_with_migrate<D, C, M>(
    connect_params: &C, migrate_params: &M,
) -> Result<D::BuildResult, D::Error>
where
    D: DatabaseInitialConnect<C>
        + DatabaseInitialMigration<M>
        + DatabaseInitial,
    C: 'static,
    M: 'static,
{
    let builder = D::start_connect(connect_params).await?;
    let builder = D::apply_migration(builder, migrate_params).await?;
    D::build(builder)
}
