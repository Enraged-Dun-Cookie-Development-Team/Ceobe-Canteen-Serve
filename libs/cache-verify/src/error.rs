#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Time format Error {0}")]
    TimeFormat(#[from] chrono::ParseError),
    #[error("HeaderValue parse to Str Error {0}")]
    ToStr(#[from] http::header::ToStrError),
    #[error("Invalid Header Value {0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("Bincode 序列化失败 {0}")]
    Bincode(#[from] bincode::Error),

    #[error("字符串编码错误{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub(crate) type VerifyResult<T> = Result<T, Error>;
