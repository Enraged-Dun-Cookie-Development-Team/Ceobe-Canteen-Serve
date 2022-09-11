#![feature(generic_associated_types)]

use std::{error::Error as StdError, future::Future};

pub trait DatabaseInitialBasic {
    type Error: StdError;
    /// start up builder
    type Builder;
}

pub trait DatabaseInitialConnect: DatabaseInitialBasic {
    /// start up params;
    type ConnectParams;
    /// future returning
    type ConnectFuture<'p>: Future<Output = Result<Self::Builder, Self::Error>>
        + 'p;

    /// connect to the database
    fn start_connect(params: &Self::ConnectParams)
        -> Self::ConnectFuture<'_>;
}

pub trait DatabaseInitialMigration: DatabaseInitialBasic {
    type MigrateParams;
    type MigrateFuture<'s>: Future<Output = Result<Self::Builder, Self::Error>>
        + 's;

    /// apply the database migration
    fn apply_migration(
        self, params: &Self::MigrateParams,
    ) -> Self::MigrateFuture<'_>;
}

/// a trait that for database init itself
pub trait DatabaseInitial: DatabaseInitialBasic + Sized {
    fn build(self) -> Result<Self, Self::Error>;
}
