use crypto_str::inner_encoders::bcrypt::BcryptError;
use persistence::{admin::user::OperateError, help_crates::StatusErr};
use status_err::{http::StatusCode, ErrPrefix};

use crate::roles::AuthorizationAccessDenyError;
#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum AuthorizeError {
    #[error("JWT 解析异常 {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("密码加密、解析异常")]
    Bcrypt(#[from] BcryptError),
    #[error(transparent)]
    UserDbOperate(#[from] OperateError),
    #[error(transparent)]
    AuthorizeLevel(#[from] AuthorizationAccessDenyError),

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
    TokenInvalid,

    #[error("Token 对应信息不存在")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x0003,
        http_code = "StatusCode::NOT_FOUND"
    ))]
    TokenInfoNotFound,
}
