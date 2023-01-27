use sql_connection::database_traits::{
    database_operates::{
        sub_operate::{SubOperate, SuperOperate},
        DatabaseOperate,
    },
    get_connect::GetDatabaseConnect,
};

pub mod announcement;
pub mod app_version;
pub mod resource;
pub mod video;

pub struct SqlCeobeOperation<'c, C>(&'c C)
where
    C: GetDatabaseConnect;

impl<'p: 'c, 'c, C> SubOperate<'p, 'c> for SqlCeobeOperation<'c, C>
where
    C: 'static + GetDatabaseConnect,
{
    type Parent<'parent> = DatabaseOperate<C>where 'parent:'c;

    fn from_parent<'parent: 'c>(parent: &'p Self::Parent<'parent>) -> Self {
        Self(parent)
    }
}

pub trait ToSqlCeobeOperation<C: GetDatabaseConnect + 'static> {
    fn ceobe_operation(&self) -> SqlCeobeOperation<'_, C>;
}

impl<C> ToSqlCeobeOperation<C> for DatabaseOperate<C>
where
    C: GetDatabaseConnect + 'static,
{
    fn ceobe_operation(&self) -> SqlCeobeOperation<'_, C> { self.child() }
}
