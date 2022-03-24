use crate::{status_code, ErrPrefix, StatusErr};

impl StatusErr for sea_orm::DbErr {

    #[inline]
    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn status(&self) -> status_code::StatusCode {
        status_code::StatusCode::new(
            ErrPrefix::SEA_ORM,
            match self {
                sea_orm::DbErr::Conn(_) => 0001,
                sea_orm::DbErr::Exec(_) => 0002,
                sea_orm::DbErr::Query(_) => 0003,
                sea_orm::DbErr::RecordNotFound(_) => 0004,
                sea_orm::DbErr::Custom(_) => 0005,
                sea_orm::DbErr::Type(_) => 0006,
            },
        )
    }
}
