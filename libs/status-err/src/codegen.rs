#[macro_export]
macro_rules! status_error {
    ($v:vis $name:ident[$pre:expr, $code:literal : $status:expr]=>$des:literal) => {
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

    ($v:vis $name:ident[$pre:expr , $code:literal]=>$des:literal)=>{
        $crate::status_error!($v $name[$code:http::StatusCode::INTERNAL_SERVER_ERROR]=>$des);
    };
    ($t:ty[$pre:expr , $code:literal: $status:expr])=>{
        impl $crate::StatusErr for $t{
            fn prefix(&self)->$crate::ErrPrefix{
                 $pre 
            }
            fn code(&self)->u16{
                $code
            }
            fn http_code(&self) -> http::StatusCode {
                $status
            }
        }
    };

    ($t:ty[$pre:expr , $code:literal])=>{
        $crate::status_error!($t[$pre $code:http::StatusCode::INTERNAL_SERVER_ERROR]);
    };
}
