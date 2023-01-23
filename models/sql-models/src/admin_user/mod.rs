mod checkers;
mod models;
mod operate;

pub use checkers::CheckError;
pub mod username {
    pub use super::checkers::username::UsernameChecker as Checker;
    pub type Uncheck = checker::Uncheck<Checker>;
    pub type Checked = checker::Checked<Checker>;
}

pub use models::{
    auth_level::AuthLevel,
    user::{ActiveModel, Column, Model, Relation},
};

pub mod sub_models {
    pub use super::models::user::{UserList, VerifiedUser};
}

pub use operate::{OperateError, UserOperate};
use sql_connection::database_traits::{
    database_operates::{sub_operate::SuperOperate, DatabaseOperate},
    get_connect::GetDatabaseConnect,
};

pub trait ToSqlUserOperate<C> {
    fn user<'c>(&'c mut self) -> UserOperate<'c, C>;
}

impl<C: GetDatabaseConnect> ToSqlUserOperate<C> for DatabaseOperate<C> {
    fn user<'c>(&'c mut self) -> UserOperate<'c, C> {
        self.child()
    }
}
