#[macro_export]
macro_rules! status_error {
    {$v:vis $name:ident[
        $pre:expr, $code:literal : $status:expr
    ]
    =>$des:literal
} => {
        #[derive(Debug,Clone)]
        $v struct $name;

        impl std::fmt::Display for $name{
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{} Error : {}",stringify!($name),$des)
            }
        }
        impl std::error::Error for $name{}

        impl $crate::StatusErr for $name {
            #[inline]
            fn prefix(&self)->$crate::ErrPrefix{
                $pre
            }
            #[inline]
            fn code(&self)->u16{
                $code
            }
            #[inline]
            fn http_code(&self) -> http::StatusCode {
                $status
            }
        }
    };

    ($v:vis $name:ident[$pre:expr , $code:literal]=>$des:literal)=>{
        $crate::status_error!($v $name[$pre,$code:$pre.get_status()]=>$des);
    };
    ($t:ty[$pre:expr , $code:literal: $status:expr])=>{
        impl $crate::StatusErr for $t{
            #[inline]
            fn prefix(&self)->$crate::ErrPrefix{
                 $pre 
            }
            #[inline]
            fn code(&self)->u16{
                $code
            }
            #[inline]
            fn http_code(&self) -> http::StatusCode {
                $status
            }
        }
    };

    ($t:ty[$pre:expr , $code:literal])=>{
        $crate::status_error!($t[$pre , $code:$pre.get_status()]);
    };
}
