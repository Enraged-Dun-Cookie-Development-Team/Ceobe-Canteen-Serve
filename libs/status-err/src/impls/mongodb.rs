use std::ops::Deref;

use mongodb::error::ErrorKind;

use crate::{ErrPrefix, HttpCode, StatusErr};

impl StatusErr for mongodb::error::Error {
    fn prefix(&self) -> crate::ErrPrefix { ErrPrefix::MONGO_DB }

    fn code(&self) -> u16 {
        match self.kind.deref() {
            ErrorKind::InvalidArgument { .. } => 1,
            ErrorKind::Authentication { .. } => 2,
            ErrorKind::BsonDeserialization(_) => 3,
            ErrorKind::BsonSerialization(_) => 4,
            ErrorKind::BulkWrite(_) => 5,
            ErrorKind::Command(_) => 6,
            ErrorKind::DnsResolve { .. } => 7,
            ErrorKind::Internal { .. } => 8,
            ErrorKind::Io(_) => 9,
            ErrorKind::ConnectionPoolCleared { .. } => 10,
            ErrorKind::InvalidResponse { .. } => 11,
            ErrorKind::ServerSelection { .. } => 12,
            ErrorKind::SessionsNotSupported => 13,
            ErrorKind::InvalidTlsConfig { .. } => 14,
            ErrorKind::Write(_) => 15,
            ErrorKind::Transaction { .. } => 16,
            ErrorKind::IncompatibleServer { .. } => 17,
            _ => 0,
        }
    }

    fn information(&self) -> std::borrow::Cow<'static, str> {
        format!("{} : {}", std::any::type_name::<Self>(), self).into()
    }

    fn http_code(&self) -> HttpCode { self.prefix().get_status() }
}
