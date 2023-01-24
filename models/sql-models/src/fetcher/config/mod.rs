use sql_connection::database_traits::database_operates::sub_operate::SuperOperate;

use super::FetcherOperate;

pub mod checkers;
pub mod models;
pub mod operate;

impl<'c, C> FetcherOperate<'c, C> {
    pub fn config(&'c mut self) -> operate::Config<'c, C> {
        self.child()
    }
}
