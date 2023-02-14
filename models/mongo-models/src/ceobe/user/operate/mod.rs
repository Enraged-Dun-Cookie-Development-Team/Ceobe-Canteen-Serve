pub mod verify;
pub mod delete;
pub mod retrieve;
pub mod update;
pub mod create;

use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;

use mongo_connection::{database_traits::{database_operates::{sub_operate::{SubOperate, SuperOperate}, DatabaseOperate}, get_connect::GetDatabaseCollection}, MongoDbError};

use super::models::UserModel;


pub struct UserOperate<'db, Conn>(&'db Conn)
where 
    Conn: GetDatabaseCollection<UserModel>;


impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: GetDatabaseCollection<UserModel>,
{
    pub(self) fn get_collection(
        &self,
    ) -> Result<Conn::CollectGuard<'db>, Conn::Error> {
        self.0.get_collection()
    }
}


impl<'db, Conn> SubOperate<'db> for UserOperate<'db, Conn>
where
Conn: GetDatabaseCollection<UserModel>
{
    type Parent = DatabaseOperate<Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self {
        Self(parent)
    }
}

pub trait ToUserOperate<Conn: GetDatabaseCollection<UserModel>> {
    fn user(&self) -> UserOperate<'_, Conn>;
}

impl<Conn> ToUserOperate<Conn> for DatabaseOperate<Conn>
where
Conn: GetDatabaseCollection<UserModel>,
{
    fn user(&self) -> UserOperate<'_, Conn> { self.child() }
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("Mongo数据库异常: {0}")]
    Db(#[from] MongoDbError),

    #[error("用户Mob ID:[{0:?}] 已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0008,
        http_code = "HttpCode::CONFLICT"
    ))]
    UserMobIdExist(String),
}