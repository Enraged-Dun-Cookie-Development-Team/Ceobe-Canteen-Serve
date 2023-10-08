use checker::prefabs::version_checker::VersionInvalidError;
use status_err::StatusErr;

use super::models::{DownloadResource, PluginVersionChecked, SpareLink};

pub mod download_resource_checker;
pub mod plugin_version_checker;
pub mod spare_link_checker;

#[derive(Debug, thiserror::Error, StatusErr)]
pub enum CheckError {
    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    VersionInvalidError(#[from] VersionInvalidError),

    #[error("长度超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),
}
