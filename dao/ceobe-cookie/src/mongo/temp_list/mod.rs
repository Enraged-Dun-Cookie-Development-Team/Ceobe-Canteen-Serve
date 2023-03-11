pub mod retrieve;
use std::ops::Deref;

use db_ops_prelude::{
    database_operates::sub_operate::{SubOperate, SuperOperate},
    mongo_connection::MongoDbError,
    ErrPrefix, HttpCode, StatusErr, ThisError,
};

use crate::CookieDatabaseOperate;

pub struct TempListOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> SubOperate<'db> for TempListOperate<'db, Conn> {
    type Parent = CookieDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> Deref for TempListOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

#[derive(Debug, ThisError, StatusErr)]
pub enum OperateError {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),

    #[error("CookieId错误")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x001B,
        http_code = "HttpCode::CONFLICT",
        resp_msg = "饼id错误，请检查"
    ))]
    CookieIdError(String),
}

type OperateResult<T> = Result<T, OperateError>;

impl<'db, Conn> CookieDatabaseOperate<'db, Conn> {
    pub fn temp_list(&self) -> TempListOperate<'_, Conn> {
        self.child()
    }
}
