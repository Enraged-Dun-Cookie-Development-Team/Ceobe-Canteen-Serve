use sql_connection::database_traits::database_operates::sub_operate::SuperOperate;

use super::FetcherOperate;

pub mod checkers;
pub mod models;
pub mod operate;
impl<'c, C: 'static> FetcherOperate<'c, C> {
    pub fn platform(&'c self) -> operate::Platform<'c, C> { self.child() }
}
