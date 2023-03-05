use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

pub struct AdminDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for AdminDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for AdminDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToAdmin<C> {
    fn admin(&self) -> AdminDatabaseOperate<'_, C>;
}

impl<C> ToAdmin<C> for DatabaseOperate<C> {
    fn admin(&self) -> AdminDatabaseOperate<'_, C> { self.child() }
}
