crate::error_generate!(
    pub DatabaseError
    Orm=OrmError
    Url=UrlParseError
);

crate::error_generate!(pub OrmError(sea_orm::DbErr));
rresult::coded_error!(OrmError[4001]);

crate::error_generate!(pub UrlParseError(url::ParseError));
rresult::coded_error!(UrlParseError[4002]);