use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

pub struct FetcherDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for FetcherDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for FetcherDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToFetcher<C> {
    fn fetcher(&self) -> FetcherDatabaseOperate<'_, C>;
}

impl<C> ToFetcher<C> for DatabaseOperate<C> {
    fn fetcher(&self) -> FetcherDatabaseOperate<'_, C> { self.child() }
}
