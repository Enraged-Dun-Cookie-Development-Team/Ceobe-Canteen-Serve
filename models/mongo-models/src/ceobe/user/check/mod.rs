use std::convert::Infallible;

use status_err::{ErrPrefix, StatusErr};

pub mod user_checker;



#[derive(Debug, thiserror::Error, StatusErr)]
pub enum CheckError {
    #[error("版本号格式错误: {0:?}")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x000A))]
    VersionFormat(String),

    #[error("长度超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
