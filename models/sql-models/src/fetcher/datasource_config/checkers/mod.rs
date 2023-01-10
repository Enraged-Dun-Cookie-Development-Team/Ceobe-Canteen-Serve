pub mod datasource_config_data;
pub mod unique_key;
pub mod unique_key_checker;

use std::convert::Infallible;

use status_err::ErrPrefix;
use status_err::StatusErr;
use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("Datasource Unique key[{0:?}] 未找到或非u64")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x0017u16))]
    UniqueKeyInValid(String),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self {
        unreachable!("enter Infallible error")
    }
}

pub use datasource_config_data::{
    FetcherDatasourceConfigUncheck, PreCheckFetcherDatasourceConfigChecker,FetcherDatasourceConfigChecker
};
pub use unique_key::DatasourceUnique;
pub use unique_key_checker::{
    FetcherDatasourceConfig, PreCheckFetcherDatasourceConfig,
    UniqueKeyChecker,
};
