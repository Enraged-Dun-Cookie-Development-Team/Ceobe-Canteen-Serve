pub mod codegen;
pub mod impls;
pub mod status_code;
use std::borrow::Cow;

use http::StatusCode;

pub trait StatusErr: std::error::Error {
    #[inline]
    fn information(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
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
    fn http_code(&self) -> StatusCode {
        self.status().http_code()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ErrPrefix(char, http::StatusCode);

impl std::fmt::Display for ErrPrefix {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ErrPrefix {
    #[inline]
    pub const fn new(sign: char, status: http::StatusCode) -> Self {
        ErrPrefix(sign, status)
    }
    #[inline]
    pub fn into_inner(self) -> char {
        self.0
    }
    #[inline]
    pub fn get_status(&self) -> http::StatusCode {
        self.1.clone()
    }

    pub const NO_ERR: Self = Self('0', StatusCode::OK);
    /// actix 框架产生的异常
    pub const ACTIX: Self = Self('F', StatusCode::INTERNAL_SERVER_ERROR);
    /// 数据库产生的异常
    pub const SEA_ORM: Self = Self('D', StatusCode::INTERNAL_SERVER_ERROR);
    /// MongoDb 数据库异常
    pub const MONGO_DB: Self = Self::new('G', StatusCode::INTERNAL_SERVER_ERROR);
    /// IO 过程中异常
    pub const IO: Self = Self('I', StatusCode::INTERNAL_SERVER_ERROR);
    ///  类型钻换时出现的异常
    pub const PARSE: Self = Self('P', StatusCode::NOT_ACCEPTABLE);
    /// 数据检查时产生的异常
    pub const CHECKER: Self = Self('C', StatusCode::NOT_ACCEPTABLE);
    /// 资源未改变
    pub const NOT_MODIFIED: Self = Self('M', StatusCode::NOT_MODIFIED);
    /// 资源查询异常
    pub const NOT_FOUND: Self = Self('S', StatusCode::NOT_FOUND);
    /// 权限认证异常
    pub const UNAUTHORIZED: Self = Self('A', StatusCode::UNAUTHORIZED);
}
