use crate::{ErrPrefix, StatusErr};

impl StatusErr for sea_orm::DbErr {
    #[inline]
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }
    #[inline]
    fn prefix(&self) -> ErrPrefix {
        ErrPrefix::SEA_ORM
    }
    #[inline]
    fn code(&self) -> u16 {
        match self {
            sea_orm::DbErr::Conn(_) => 0001,
            sea_orm::DbErr::Exec(_) => 0002,
            sea_orm::DbErr::Query(_) => 0003,
            sea_orm::DbErr::RecordNotFound(_) => 0004,
            sea_orm::DbErr::Custom(_) => 0005,
            sea_orm::DbErr::Type(_) => 0006,
        }
    }
}
