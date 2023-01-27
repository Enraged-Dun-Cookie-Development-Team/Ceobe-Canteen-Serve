use sql_connection::database_traits::{
    database_operates::{
        sub_operate::{SubOperate, SuperOperate},
        DatabaseOperate,
    },
    get_connect::GetDatabaseConnect,
};

pub mod config;
pub mod datasource_config;
pub mod global_config;
pub mod platform_config;

pub struct FetcherOperate<'c, C>(&'c C);

impl<'c, C> SubOperate<'c> for FetcherOperate<'c, C>
where
    C: GetDatabaseConnect,
{
    type Parent = DatabaseOperate<C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent) }
}

pub trait ToFetcherOperate<C: 'static> {
    fn fetcher_operate<'s, 'c>(&'s self) -> FetcherOperate<'c, C>
    where
        Self: 's,
        's: 'c;
}

impl<C: 'static + GetDatabaseConnect> ToFetcherOperate<C>
    for DatabaseOperate<C>
{
    fn fetcher_operate<'s, 'c>(&'s self) -> FetcherOperate<'c, C>
    where
        Self: 's,
        's: 'c,
    {
        self.child()
    }
}
