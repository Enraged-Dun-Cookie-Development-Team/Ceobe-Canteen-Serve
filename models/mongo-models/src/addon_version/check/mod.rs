use std::convert::Infallible;

pub mod version_checker;

#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("Url格式异常 {0}")]
    Url(#[from] url::ParseError),

    #[error("版本号格式错误: {0:?}")]
    VersionFormat(String),
}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!() }
}
