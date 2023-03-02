use std::ops::Deref;

use abstract_database::ceobe::CeobeDatabaseOperate;
use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};

#[path = "mongo/user/mod.rs"] pub mod user;

pub struct UserDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for UserDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for UserDatabaseOperate<'db, Conn> {
    type Parent = CeobeDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCeobeUser<C> {
    fn user(&self) -> UserDatabaseOperate<'_, C>;
}

impl<C> ToCeobeUser<C> for CeobeDatabaseOperate<'_, C> {
    fn user(&self) -> UserDatabaseOperate<'_, C> { self.child() }
}
