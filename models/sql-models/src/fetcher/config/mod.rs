use sql_connection::database_traits::database_operates::sub_operate::SuperOperate;

use super::FetcherOperate;

pub mod checkers;
pub mod models;
pub mod operate;

impl<'s, C: 'static> FetcherOperate<'s, C> {
    pub fn config<'c>(&'s self) -> operate::Config<'c, C>
    where
        's: 'c,
        Self: 's,
    {
        self.child()
    }
}
