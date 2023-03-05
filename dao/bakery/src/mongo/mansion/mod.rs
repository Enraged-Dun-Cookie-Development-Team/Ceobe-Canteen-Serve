use mongo_connection::{
    database_traits::{
        database_operates::{
            sub_operate::{SubOperate, SuperOperate},
            DatabaseOperate,
        },
        get_connect::GetDatabaseCollection,
    },
    MongoDbError,
};

mod create;
mod delete;
mod retrieve;
mod update;
mod verify;
use status_err::{ErrPrefix, HttpCode};
use thiserror::Error;
pub use OperateError::*;

use super::preludes::ModelMansion;
pub type MongoErr = mongodb::error::Error;
pub struct MansionOperate<'db, Db>(&'db Db)
where
    Db: GetDatabaseCollection<ModelMansion> + 'db;

impl<'db, Db> MansionOperate<'db, Db>
where
    Db: GetDatabaseCollection<ModelMansion>,
{
    pub(self) fn get_collection(
        &self,
    ) -> Result<Db::CollectGuard<'db>, Db::Error> {
        self.0.get_collection()
    }
}

impl<'db, Db> SubOperate<'db> for MansionOperate<'db, Db>
where
    Db: GetDatabaseCollection<ModelMansion> + 'db,
{
    type Parent = DatabaseOperate<Db>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToMansionOperate<Db: GetDatabaseCollection<ModelMansion>> {
    fn mansion(&self) -> MansionOperate<'_, Db>;
}

impl<Db> ToMansionOperate<Db> for DatabaseOperate<Db>
where
    Db: GetDatabaseCollection<ModelMansion>,
{
    fn mansion(&self) -> MansionOperate<'_, Db> { self.child() }
}

#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

#[derive(Debug, Error, status_err::StatusErr)]
pub enum OperateError {
    #[error("Mongo数据库异常")]
    Db(#[from] MongoDbError),

    #[error("指定饼学大厦ID未找到")]
    #[status_err(err(prefix = "ErrPrefix::NOT_FOUND", err_code = 0x0001,))]
    MansionNotFound,
    #[error("指定ID:[{0:?}] 的饼学大厦已经存在")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x0008,
        http_code = "HttpCode::CONFLICT"
    ))]
    MansionIdExist(String),
}
