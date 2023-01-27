mod checkers;
mod models;
mod operate;

pub use checkers::{
    app_version_checker::AppVersionChecker,
    app_version_data::{
        CeobeOperationAppVersion as Checked,
        CeobeOperationAppVersionChecker as Checker,
        CeobeOperationAppVersionUncheck as Unchecked,
    },
    CheckError,
};
pub use models::model_app_version::{
    ActiveModel, Column, Entity, Model, Relation,
};
pub use operate::{AppVersionOperate as Operate, OperateError};
use sql_connection::database_traits::{
    database_operates::sub_operate::SuperOperate,
    get_connect::GetDatabaseConnect,
};

use super::SqlCeobeOperation;

impl<'c, C: GetDatabaseConnect + 'static> SqlCeobeOperation<'c, C> {
    pub fn app_version(&'c  self) -> Operate<'c, C> { self.child() }
}
