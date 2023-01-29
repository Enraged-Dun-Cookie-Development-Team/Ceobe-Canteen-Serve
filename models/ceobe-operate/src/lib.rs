use std::ops::Deref;

use db_ops_prelude::database_operates::{
    sub_operate::{SubOperate, SuperOperate},
    DatabaseOperate,
};
#[path="sql/announcement/mod.rs"]
pub mod announcement;
#[path ="sql/app_version/mod.rs"]
pub mod app_version;
#[path ="mongo/plugin_version/mod.rs"]
pub mod plugin_version;
#[path ="sql/resource/mod.rs"]
pub mod resource;
#[path ="sql/video/mod.rs"]
pub mod video;
pub struct CeobeDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for CeobeDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for CeobeDatabaseOperate<'db, Conn> {
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCeobeOperation<C> {
    fn ceobe_operation(&self) -> CeobeDatabaseOperate<'_, C>;
}

impl<C> ToCeobeOperation<C> for DatabaseOperate<C> {
    fn ceobe_operation(&self) -> CeobeDatabaseOperate<'_, C> { self.child() }
}
