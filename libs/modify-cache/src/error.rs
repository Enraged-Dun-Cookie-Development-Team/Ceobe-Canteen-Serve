use status_err::StatusErr;

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
}

pub(crate) type VerifyResult<T> = Result<T, Error>;

impl StatusErr for Error {
    fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Error::TimeFormat(inner) => inner.respond_msg(),
            Error::ToStr(inner) => inner.respond_msg(),
            Error::InvalidHeaderValue(inner) => inner.respond_msg(),
            Error::Bincode(inner) => inner.respond_msg(),
        }
    }

    fn prefix(&self) -> status_err::ErrPrefix {
        match self {
            Error::TimeFormat(inner) => inner.prefix(),
            Error::ToStr(inner) => inner.prefix(),
            Error::InvalidHeaderValue(inner) => inner.prefix(),
            Error::Bincode(inner) => inner.prefix(),
        }
    }

    fn code(&self) -> u16 {
        match self {
            Error::TimeFormat(inner) => inner.code(),
            Error::ToStr(inner) => inner.code(),
            Error::InvalidHeaderValue(inner) => inner.code(),
            Error::Bincode(inner) => inner.code(),
        }
    }
}
