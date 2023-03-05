use std::fmt::Debug;

use crypto_str::inner_encoders::bcrypt::BcryptError;
use http::StatusCode;
use admin::user::OperateError;
use status_err::{ErrPrefix, StatusErr};

use crate::utils::user_authorize::auth_level;

#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum AuthorizeError {
    #[error("JWT 解析异常 {0}")]
    Jwt(#[from] jwt::Error),
    #[error("密码加密、解析异常")]
    Bcrypt(#[from] BcryptError),
    #[error(transparent)]
    UserDbOperate(#[from] OperateError),
    #[error(transparent)]
    AuthorizeLevel(#[from] auth_level::UnacceptableAuthorizationLevelError),

    #[error("Token 信息未找到")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x0001
    ))]
    TokenNotFound,

    #[error("Token 已经失效")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x0006
    ))]
    TOkenInvalid,

    #[error("Token 对应信息不存在")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x0003,
        http_code = "StatusCode::NOT_FOUND"
    ))]
    TokenInfoNotFound,

    #[error("缺少Authorize鉴权中间件")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x000A,
        http_code = "StatusCode::INTERNAL_SERVER_ERROR"
    ))]
    NoAuthorizeLayer,
}
