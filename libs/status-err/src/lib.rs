pub mod codegen;
mod impls;
pub mod status_code;
use std::borrow::Cow;

pub use status_err_derive::StatusErr;
pub use thiserror::Error as ThisError;

pub trait StatusErr: std::error::Error {
    #[inline]
    fn information(&self) -> Cow<'_, str> { format!("{self}").into() }

    fn respond_msg(&self) -> Cow<'_, str> { self.information() }
    /// 异常码
    /// 用于唯一标记某一类型异常
    fn prefix(&self) -> ErrPrefix;

    fn code(&self) -> u16;
    ///
    fn status(&self) -> status_code::StatusCode {
        status_code::StatusCode::new(self.prefix(), self.code())
    }
    /// 对应的http状态码
    #[inline]
    fn http_code(&self) -> HttpCode { self.status().http_code() }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrPrefix(char, http::StatusCode);
pub use http::StatusCode as HttpCode;

impl std::fmt::Display for ErrPrefix {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ErrPrefix {
    /// 数据检查时产生的异常
    pub const CHECKER: Self = Self('C', HttpCode::BAD_REQUEST);
    /// IO 过程中异常
    pub const IO: Self = Self('I', HttpCode::INTERNAL_SERVER_ERROR);
    pub const LOGGER_REPORT: Self =
        Self('L', HttpCode::INTERNAL_SERVER_ERROR);
    /// MongoDb 数据库异常
    pub const MONGO_DB: Self =
        Self::new('G', HttpCode::INTERNAL_SERVER_ERROR);
    /// 资源查询异常
    pub const NOT_FOUND: Self = Self('S', HttpCode::NOT_FOUND);
    /// 资源未改变
    pub const NOT_MODIFIED: Self = Self('M', HttpCode::NOT_MODIFIED);
    pub const NO_ERR: Self = Self('0', HttpCode::OK);
    ///  类型钻换时出现的异常
    pub const PARSE: Self = Self('P', HttpCode::BAD_REQUEST);
    /// 七牛云上传异常
    pub const QI_NIU: Self = Self('Q', HttpCode::BAD_REQUEST);
    /// Redis 数据库异常
    pub const REDIS_DB: Self =
        Self::new('R', HttpCode::INTERNAL_SERVER_ERROR);
    /// 数据库产生的异常
    pub const SEA_ORM: Self = Self('D', HttpCode::INTERNAL_SERVER_ERROR);
    /// actix 框架产生的异常
    pub const SERVE: Self = Self('F', HttpCode::INTERNAL_SERVER_ERROR);
    /// 权限认证异常
    pub const UNAUTHORIZED: Self = Self('A', HttpCode::UNAUTHORIZED);

    #[inline]
    pub const fn new(sign: char, status: HttpCode) -> Self {
        ErrPrefix(sign, status)
    }

    #[inline]
    pub fn into_inner(self) -> char { self.0 }

    #[inline]
    pub fn get_status(&self) -> http::StatusCode { self.1 }
}
