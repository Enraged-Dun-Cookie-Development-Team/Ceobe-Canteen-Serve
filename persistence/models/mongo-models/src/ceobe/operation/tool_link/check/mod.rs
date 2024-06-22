mod tool_link_checker;

use checker::prefabs::version_checker::VersionInvalidError;
use status_err::{StatusErr};

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum CheckError {
    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("长度超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),
}
