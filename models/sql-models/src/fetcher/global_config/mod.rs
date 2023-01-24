use sql_connection::database_traits::database_operates::sub_operate::SuperOperate;

use super::FetcherOperate;

pub mod checkers;
pub mod models;
pub mod operate;
impl<'c, C> FetcherOperate<'c, C> {
    pub fn global(&'c mut self) -> operate::Global<'c, C> {
        self.child()
    }
}
