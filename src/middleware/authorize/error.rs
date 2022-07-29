use std::fmt::Debug;

use bcrypt::BcryptError;
use http::StatusCode;
use orm_migrate::sql_models::admin_user::operate::OperateError;
use serde::Serialize;
use status_err::{ErrPrefix, StatusErr};

use crate::utils::user_authorize::auth_level;

#[derive(Debug, thiserror::Error)]
pub enum AuthorizeError {
    #[error("JWT 解析异常 {0}")]
    Jwt(#[from] jwt::Error),
    #[error("密码加密、解析异常")]
    Bcrypt(#[from] BcryptError),
    #[error(transparent)]
    UserDbOperate(#[from] OperateError),
    #[error(transparent)]
    AuthorizeLevel(#[from] auth_level::UnacceptableAuthorizationLevelError),

    #[error("token 信息未找到")]
    NoToken,
    #[error("token 已经失效")]
    TOkenInvalid,
    #[error("token 对应的信息不存在")]
    TokenInfoNotFound,

    #[error("缺少Authorize鉴权中间件")]
    NoAuthorizeLayer,
}

impl Serialize for AuthorizeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("Error => {}", self))
    }
}

impl StatusErr for AuthorizeError {
    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            AuthorizeError::Jwt(inner) => inner.prefix(),
            AuthorizeError::Bcrypt(inner) => inner.prefix(),
            AuthorizeError::UserDbOperate(inner) => inner.prefix(),
            AuthorizeError::AuthorizeLevel(inner) => inner.prefix(),
            AuthorizeError::NoToken
            | AuthorizeError::TOkenInvalid
            | AuthorizeError::NoAuthorizeLayer
            | AuthorizeError::TokenInfoNotFound => ErrPrefix::UNAUTHORIZED,
        }
    }

    fn code(&self) -> u16 {
        match self {
            AuthorizeError::Jwt(inner) => inner.code(),
            AuthorizeError::Bcrypt(inner) => inner.code(),
            AuthorizeError::UserDbOperate(inner) => inner.code(),
            AuthorizeError::AuthorizeLevel(inner) => inner.code(),
            AuthorizeError::NoToken => 0x0001,
            AuthorizeError::TOkenInvalid => 0x0003,
            AuthorizeError::TokenInfoNotFound => 0x0006,
            AuthorizeError::NoAuthorizeLayer => 0x000A,
        }
    }

    fn http_code(&self) -> http::StatusCode {
        match self {
            AuthorizeError::TokenInfoNotFound => StatusCode::NOT_FOUND,
            AuthorizeError::NoAuthorizeLayer => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => self.status().http_code(),
        }
    }
}

status_err::resp_error_impl!(AuthorizeError);
