crate::error_generate!(
    pub DatabaseError
    Orm=sea_orm::DbErr
    Url=url::ParseError
);
