mod create;
mod delete;
mod retrieve;
mod verify;

use db_ops_prelude::{
    mongo_connection::{database_traits::dao_operator, MongoDbError},
    ErrPrefix, HttpCode, StatusErr, ThisError,
};

use crate::OperationDatabaseOperate;
dao_operator!(OperationDatabaseOperate=>ReleaseVersion(release_version,err=Error));

#[derive(Debug, ThisError, StatusErr)]
pub enum Error {
    #[error("数据库查询异常{0}")]
    Db(#[from] MongoDbError),

    #[error("版本号已经存在 {1:?}:{0}")]
    #[status_err(err(
        prefix = "ErrPrefix::CHECKER",
        err_code = 0x000B,
        http_code = "HttpCode::CONFLICT"
    ))]
    ConflictVersion(models::Version, models::ReleasePlatform),
    #[error("版本信息不存在 {1:?}:{0}")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::NOT_FOUND",))]
    VersionNotFind(models::Version, models::ReleasePlatform),
    #[error("暂没有版本信息")]
    #[status_err(err(err_code = 0x0005, prefix = "ErrPrefix::NOT_FOUND",))]
    VersionInfoNoExist,
    #[error("新版本的版本号过旧, 新版本号应大于 {1:?}:{0}")]
    #[status_err(err(
        err_code = 0x00017,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::CONFLICT"
    ))]
    VersionTooOld(models::Version, models::ReleasePlatform),

    #[error("Bson序列化失败")]
    #[status_err(err(err_code = 0x0004, prefix = "ErrPrefix::MONGO_DB",))]
    Bson(#[from] db_ops_prelude::mongodb::bson::ser::Error),
}

use db_ops_prelude::mongo_models::ceobe::operation::version::*;
