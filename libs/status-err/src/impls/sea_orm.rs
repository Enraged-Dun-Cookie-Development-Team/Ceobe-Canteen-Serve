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
            sea_orm::DbErr::Conn(_) => 0x00_01,
            sea_orm::DbErr::Exec(_) => 0x00_02,
            sea_orm::DbErr::Query(_) => 0x00_03,
            sea_orm::DbErr::RecordNotFound(_) => 0x00_04,
            sea_orm::DbErr::Custom(_) => 0x00_05,
            sea_orm::DbErr::Type(_) => 0x00_06,
            sea_orm::DbErr::Json(_) => 0x00_07,
            sea_orm::DbErr::Migration(_) => 0x00_08,
        }
    }
}
