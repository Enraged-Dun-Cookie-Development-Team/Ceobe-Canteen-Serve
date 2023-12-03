use std::convert::Infallible;

pub use datasource_config_data::FetcherqqDatasourceConfigUncheck;
use status_err::{ErrPrefix, StatusErr};
use thiserror::Error;
pub use unique_key::DatasourceUnique;
pub use unique_key_checker::{FetcherDatasourceConfig, UniqueKeyChecker};
pub use CheckError::*;

mod datasource_config_data;
mod unique_key;
mod unique_key_checker;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    #[error("范围超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),

    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("Datasource Unique key[{0:?}] 未找到")]
    #[status_err(err(prefix = "ErrPrefix::CHECKER", err_code = 0x0017u16))]
    UniqueKeyInvalid(String),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!("enter Infallible error") }
}
