mod checkers;
mod models;
mod operate;

pub use checkers::{
    video_data::{
        CeobeOpVideo as Checked, CeobeOpVideoChecker as Checker,
        CeobeOpVideoUncheck as Uncheck,
    },
    CheckError,
};
pub use models::model_video::{ActiveModel, Column, Entity, Model, Relation};

pub mod bv {
    pub use std::string::String as Uncheck;

    pub use super::checkers::bv::{Bv as Checked, BvChecker as Checker};
    use super::checkers::bv_arg_checker;
    pub mod query {
        pub use super::bv_arg_checker::{
            BvQuery as Checked, BvQueryChecker as Checker,
            BvQueryUncheck as Uncheck,
        };
    }
}

pub use operate::{OperateError, VideoOperate as Operate};
use sql_connection::database_traits::{
    database_operates::sub_operate::SuperOperate,
    get_connect::GetDatabaseConnect,
};

use super::SqlCeobeOperation;

impl<'c, C: GetDatabaseConnect> SqlCeobeOperation<'c, C> {
    pub fn video(&'c mut self) -> Operate<'c, C> { self.child() }
}
