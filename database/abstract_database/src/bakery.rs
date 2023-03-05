use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

pub struct BakeryDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for BakeryDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for BakeryDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToBakery<C> {
    fn bakery(&self) -> BakeryDatabaseOperate<'_, C>;
}

impl<C> ToBakery<C> for DatabaseOperate<C> {
    fn bakery(&self) -> BakeryDatabaseOperate<'_, C> { self.child() }
}
