use sql_connection::database_traits::{database_operates::{sub_operate::{SubOperate, SuperOperate}, DatabaseOperate}, get_connect::GetDatabaseConnect};

pub mod config;
pub mod datasource_config;
pub mod global_config;
pub mod platform_config;

pub struct FetcherOperate<'c,C>(&'c C);

impl<'c, C:GetDatabaseConnect> SubOperate<'c> for FetcherOperate<'c, C> {
    type Parent = DatabaseOperate<C>;

    fn from_parent(parent: &'c mut Self::Parent) -> Self {
        Self(parent)
    }
}

pub trait ToFetcherOperate<C> {
    fn fetcher_operate<'c>(&'c mut self)->FetcherOperate<'c,C>;
}

impl<C:GetDatabaseConnect> ToFetcherOperate<C> for DatabaseOperate<C> {
    fn fetcher_operate<'c>(&'c mut self)->FetcherOperate<'c,C> {
        self.child()
    }
}