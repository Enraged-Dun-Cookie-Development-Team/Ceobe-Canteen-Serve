#[macro_export]
/// 辅助构造Status error 的宏
/// 1. 新建Unit 类型的异常，并为其实现[crate::StatusErr](crate::StatusErr)
///     ```rust
///         status_error!{
///            //|----------新建异常类型的可见性
///            //|      |------- 新建异常的类型名称
///             pub BadFraction
///             [
///                 ErrPrefix::CHECKER, //  新建类型的前缀码类型
///                 0003: StatusCode::NOT_FOUND
///                 // |        |------新建异常的Http状态码(可以省略，如果不提供，将使用前缀码默认http状态码)
///                 // |-------------- 新建类型的异常标识码(唯一)
///             ]=>"错误的Fraction值范围(0~5)"
///                        // |--------新建异常的描述内容
///         }
///     ```
/// 2. 为已有类型实现 [crate::StatusErr](crate::StatusErr)
///     ```rust
///         status_error!(
///               // |-------------已有类型名称
///             std::io::Error
///             [
///                 ErrPrefix::IO,                      // 为已有类型实现时使用的前缀码类型
///                 0001:                               // 为已有类型实现时使用的异常标识码(唯一)
///                 StatusCode::INTERNAL_SERVER_ERROR   // 为已有类型实现时使用的Http状态码(可以省略，如果不提供，将使用前缀码默认http状态码)
///             ]
///         );
///     ```
macro_rules! status_error {
    {
        $v:vis $name:ident
        [
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

    {
        $v:vis $name:ident
        [
            $pre:expr ,
            $code:literal
        ]=>$des:literal
    }=>{
        $crate::status_error!($v $name[$pre,$code:$pre.get_status()]=>$des);
    };
    ($t:ty[$pre:expr, $code:literal: $status:expr])=>{
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
