use std::ops::Deref;

use abstract_database::ceobe::CeobeDatabaseOperate;
use db_ops_prelude::database_operates::sub_operate::{
    SubOperate, SuperOperate,
};
#[path = "sql/announcement/mod.rs"] pub mod announcement;
#[path = "sql/app_version/mod.rs"] pub mod app_version;
#[path = "mongo/plugin_version/mod.rs"]
pub mod plugin_version;
#[path = "sql/resource/mod.rs"] pub mod resource;
#[path = "sql/video/mod.rs"] pub mod video;
pub struct OperationDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for OperationDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for OperationDatabaseOperate<'db, Conn> {
    type Parent = CeobeDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCeobeOperation<C> {
    fn operation(&self) -> OperationDatabaseOperate<'_, C>;
}

impl<C> ToCeobeOperation<C> for CeobeDatabaseOperate<'_, C> {
    fn operation(&self) -> OperationDatabaseOperate<'_, C> { self.child() }
}
