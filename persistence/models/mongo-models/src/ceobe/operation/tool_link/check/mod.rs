pub mod tool_link_checker;

use std::{convert::Infallible};
use mongodb::bson;
use status_err::{StatusErr};

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum CheckError {
    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("长度超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error(transparent)]
    BsonUuid(#[from] bson::uuid::Error),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
