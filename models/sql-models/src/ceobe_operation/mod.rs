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

impl<'c, C> SubOperate<'c> for SqlCeobeOperation<'c, C>
where
    C: GetDatabaseConnect,
{
    type Parent = DatabaseOperate<C>;

    fn from_parent(parent: &'c Self::Parent) -> Self {
        Self(parent)
    }
}

pub trait ToSqlCeobeOperation<C: GetDatabaseConnect> {
    fn ceobe_operation(&self) -> SqlCeobeOperation<'_, C>;
}

impl<C> ToSqlCeobeOperation<C> for DatabaseOperate<C>
where
    C: GetDatabaseConnect,
{
    fn ceobe_operation(&self) -> SqlCeobeOperation<'_, C> {
        self.child()
    }
}
