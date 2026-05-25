use std::convert::Infallible;

use bitmap_convert::error::Error as BitmapConvError;
use mob_push_server::MobPushError;
use persistence::{
    ceobe_cookie::{
        analyze::OperateError as AnalyzeOperateError,
        search_content::OperateError as SearchContentOperateError,
        temp_list::OperateError as TempListOperateError,
        terra_comic::OperateError as TerraComicOperateError,
    },
    ceobe_user::property::OperateError as CeobeUserOperateError,
    fetcher::{
        datasource_combination::OperateError as DatasourceCombinationOperateError,
        datasource_config::OperateError as DatasourceOperateError,
    },
    help_crates::mongodb::bson::{self, oid::ObjectId},
};
use redis::RedisError;
use status_err::{
    StatusErr,
    generated_error::{
        checker_kind::{
            MobPushError as GenMobPushError, UpdateCookieIdCacheFailureError,
        },
        serve_kind::JoinError as GenJoinError,
    },
};
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error, StatusErr)]
// TODO: 后续修复 large_enum_variant，考虑 Box 包装大变体
#[allow(clippy::large_enum_variant)]
pub enum LogicError {
    #[error(transparent)]
    TempListOperateError(#[from] TempListOperateError),

    #[error(transparent)]
    AnalyzeOperateError(#[from] AnalyzeOperateError),

    #[error(transparent)]
    BitmapConvError(#[from] BitmapConvError),

    #[error("Json 反/序列化失败 {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    CeobeUserOperateError(#[from] CeobeUserOperateError),

    #[error(transparent)]
    DatasourceOperateError(#[from] DatasourceOperateError),

    #[error(transparent)]
    SearchContentOperateError(#[from] SearchContentOperateError),

    #[error(transparent)]
    DatasourceCombinationOperateError(
        #[from] DatasourceCombinationOperateError,
    ),

    #[error(transparent)]
    TerraComicOperateError(#[from] TerraComicOperateError),

    #[error(transparent)]
    #[status_err(err(bind = "GenMobPushError"))]
    MobPushError(#[from] MobPushError),

    #[error(transparent)]
    QqChannelError(#[from] qq_channel_warning::Error),

    #[error(transparent)]
    #[status_err(err(bind = "GenJoinError"))]
    JoinError(#[from] JoinError),

    #[error("Redis异常: {0}")]
    Redis(#[from] RedisError),

    #[error("更新饼id缓存失效：{0}")]
    #[status_err(err(bind = "UpdateCookieIdCacheFailureError"))]
    UpdateCookieIdCacheFailure(ObjectId),

    #[error(transparent)]
    BsonOidErr(#[from] bson::oid::Error),
    #[error(transparent)]
    QiniuError(#[from] ceobe_qiniu_upload::Error),
}

impl From<Infallible> for LogicError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}

pub(crate) type LogicResult<T> = Result<T, LogicError>;
