use std::borrow::Cow;

use http::StatusCode;

pub trait ErrorCode: std::error::Error {
    fn description(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
    fn code(&self) -> i16;
    fn http_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn collect(&self) -> (Cow<'static, str>, i16, StatusCode) {
        (ErrorCode::description(self), self.code(), self.http_code())
    }
}

#[macro_export]
macro_rules! coded_error {
    ($v:vis $name:ident[$code:literal: $status:expr]=>$des:literal) => {
        #[derive(Debug,Clone)]
        $v struct $name;

        impl std::fmt::Display for $name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{} Error : {}",stringify!($name),$des)
            }
        }
        impl std::error::Error for $name{}

        impl $crate::ErrorCode for $name {
            fn code(&self)->i16{
                $code
            }
            fn http_code(&self) -> http::StatusCode {
                $status
            }
        }
    };

    ($v:vis $name:ident[$code:literal]=>$des:literal)=>{
        $crate::coded_error!($v $name[$code:http::StatusCode::INTERNAL_SERVER_ERROR]=>$des);
    };
    ($t:ty[$code:literal: $status:expr])=>{
        impl $crate::ErrorCode for $t{
            fn code(&self)->i16{
                $code
            }
            fn http_code(&self) -> http::StatusCode {
                $status
            }
        }
    };

    ($t:ty[$code:literal])=>{
        $crate::coded_error!($t[$code:http::StatusCode::INTERNAL_SERVER_ERROR]);
    };
}

coded_error!(actix::dev::MailboxError[4006]);
coded_error!(std::io::Error[5001]);
coded_error!(sea_orm::DbErr[3001]);
coded_error!(url::ParseError[3002]);
coded_error!(actix_web::error::Error[4001]);