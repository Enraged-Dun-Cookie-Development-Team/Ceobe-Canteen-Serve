use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

pub struct CeobeDatabaseOperate<'db, Conn>(&'db Conn);
pub struct CeobeMongoOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for CeobeDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for CeobeDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCeobe<C> {
    fn ceobe(&self) -> CeobeDatabaseOperate<'_, C>;
}

impl<C> ToCeobe<C> for DatabaseOperate<C> {
    fn ceobe(&self) -> CeobeDatabaseOperate<'_, C> { self.child() }
}
