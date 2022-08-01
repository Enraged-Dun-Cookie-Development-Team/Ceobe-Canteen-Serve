pub mod download_resource_checker;
pub mod plugin_version_checker;
pub mod spare_link_checker;

pub mod version_checker;

use status_err::{ErrPrefix, StatusErr};

pub(self) use super::models::{
    DownloadResource, PluginVersionChecked, SpareLink, Version,
};

#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("版本号格式错误: {0:?}")]
    VersionFormat(String),

    #[error("长度超出限制: {0}")]
    LengthExceed(#[from] range_limit::Error),
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            CheckError::Url(url) => url.prefix(),
            CheckError::LengthExceed(inner) => inner.prefix(),
            CheckError::VersionFormat(_) => ErrPrefix::CHECKER,
        }
    }

    fn code(&self) -> u16 {
        match self {
            CheckError::Url(inner) => inner.code(),
            CheckError::VersionFormat(_) => 0x000A,
            CheckError::LengthExceed(inner) => inner.code(),
        }
    }
}
