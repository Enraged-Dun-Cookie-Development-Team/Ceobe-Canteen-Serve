use crate::{ErrPrefix, StatusErr};

impl StatusErr for sea_orm::DbErr {
    #[inline]
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    #[inline]
    fn prefix(&self) -> ErrPrefix { ErrPrefix::SEA_ORM }

    #[inline]
    fn code(&self) -> u16 {
        match self {
            sea_orm::DbErr::Conn(_) => 1,
            sea_orm::DbErr::Exec(_) => 2,
            sea_orm::DbErr::Query(_) => 3,
            sea_orm::DbErr::RecordNotFound(_) => 4,
            sea_orm::DbErr::Custom(_) => 5,
            sea_orm::DbErr::Type(_) => 6,
            sea_orm::DbErr::Json(_) => 7,
        }
    }
}
