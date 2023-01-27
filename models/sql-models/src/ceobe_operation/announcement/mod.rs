mod checkers;
mod models;
mod operate;

pub use checkers::{
    announcement_data::{
        CeobeOpAnnouncement as Checked,
        CeobeOpAnnouncementChecker as Checker,
        CeobeOpAnnouncementUncheck as Uncheck,
    },
    CheckError,
};
pub use models::model_announcement::{
    ActiveModel, Column, Entity, Model, Relation,
};
pub use operate::{AnnouncementOperate as Operate, OperateError};
use sql_connection::database_traits::{
    database_operates::sub_operate::SuperOperate,
    get_connect::GetDatabaseConnect,
};

use super::SqlCeobeOperation;

impl<'c, C: GetDatabaseConnect + 'static> SqlCeobeOperation<'c, C> {
    pub fn announcement(&'c self) -> Operate<'c, C> {
        self.child()
    }
}
