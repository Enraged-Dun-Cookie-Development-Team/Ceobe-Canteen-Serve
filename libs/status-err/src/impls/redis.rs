use crate::{ErrPrefix, StatusErr};

impl StatusErr for redis::RedisError {
    fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
        "Redis数据库异常".into()
    }

    #[inline]
    fn prefix(&self) -> ErrPrefix { ErrPrefix::REDIS_DB }

    #[inline]
    fn code(&self) -> u16 {
        match self.kind() {
            redis::ErrorKind::ResponseError => 1,
            redis::ErrorKind::AuthenticationFailed => 2,
            redis::ErrorKind::TypeError => 3,
            redis::ErrorKind::ExecAbortError => 4,
            redis::ErrorKind::BusyLoadingError => 5,
            redis::ErrorKind::NoScriptError => 6,
            redis::ErrorKind::InvalidClientConfig => 7,
            redis::ErrorKind::Moved => 8,
            redis::ErrorKind::Ask => 9,
            redis::ErrorKind::TryAgain => 10,
            redis::ErrorKind::ClusterDown => 11,
            redis::ErrorKind::CrossSlot => 12,
            redis::ErrorKind::MasterDown => 13,
            redis::ErrorKind::IoError => 14,
            redis::ErrorKind::ClientError => 15,
            redis::ErrorKind::ExtensionError => 16,
            redis::ErrorKind::ReadOnly => 17,
            _ => 0,
        }
    }

    #[inline]
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
