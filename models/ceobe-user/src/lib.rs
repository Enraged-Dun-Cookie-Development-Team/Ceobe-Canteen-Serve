use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

#[path = "mongo/user/mod.rs"] pub mod user;

pub struct CeobeDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for CeobeDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for CeobeDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCeobeUser<C> {
    fn ceobe_user(&self) -> CeobeDatabaseOperate<'_, C>;
}

impl<C> ToCeobeUser<C> for DatabaseOperate<C> {
    fn ceobe_user(&self) -> CeobeDatabaseOperate<'_, C> { self.child() }
}
