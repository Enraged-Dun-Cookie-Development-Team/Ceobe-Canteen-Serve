#[macro_export]
/// 辅助构造Status error 的宏
/// 1. 新建Unit 类型的异常，并为其实现[crate::StatusErr](crate::StatusErr)
/// ```rust
/// status_error!{
/// //   |----------新建异常类型的可见性
/// //   |      |------- 新建异常的类型名称
///     pub BadFraction
///     [
///         ErrPrefix::CHECKER, //  新建类型的前缀码类型
///         0003: StatusCode::NOT_FOUND
/// //         |     |----新建异常的Http状态码(可以省略，如果不提供，
/// //         |          将使用前缀码默认http状态码)                 
/// //         |----------新建类型的异常标识码(唯一)             
///     ]=>"错误的Fraction值范围(0~5)"
/// //             |--------新建异常的描述内容
/// }
///     ```
/// 2. 为已有类型实现 [crate::StatusErr](crate::StatusErr)
/// ```rust
/// status_error!(
/// //       |-------------已有类型名称
///     std::io::Error
///     [
/// //         |--- 为已有类型实现时使用的前缀码类型
///         ErrPrefix::IO,                      
/// //         |--- 为已有类型实现时使用的异常标识码(唯一)
///          0001:
/// //         |--为已有类型实现时使用的Http状态码
/// //         |  (可以省略，如果不提供，
/// //         |   将使用前缀码默认http状态码)             
///         StatusCode::INTERNAL_SERVER_ERROR   
///     ]
/// );
/// ```
macro_rules! status_error {
    {
        $v:vis $name:ident
        [
        $pre:expr, $code:literal : $status:expr
        ]
        =>$des:literal | $resp_msg:literal
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
            fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
                $resp_msg.into()
            }
            #[inline]
            fn prefix(&self) -> $crate::ErrPrefix{
                $pre
            }
            #[inline]
            #[allow(clippy::zero_prefixed_literal)]
            fn code(&self) -> u16{
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
        $pre:expr, $code:literal : $status:expr
        ]
        =>$des:literal
    }=>{
        $crate::status_error!($v $name[$pre,$code:$status]=>$des |$des );
    };
    {
        $v:vis $name:ident
        [
            $pre:expr ,
            $code:literal
            ]=>$des:literal |  $resp_msg:literal
    }=>{
            $crate::status_error!($v $name[$pre,$code:$pre.get_status()]=>$des |$resp_msg );
    };
    {
        $v:vis $name:ident
        [
            $pre:expr ,
            $code:literal
            ]=>$des:literal
    }=>{
            $crate::status_error!($v $name[$pre,$code:$pre.get_status()]=>$des | $des );
    };

    ($t: ty [$pre:expr, $code:literal: $status:expr] -> $resp_msg:literal)=>{
            impl $crate::StatusErr for $t {

                #[inline]
                fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
                    $resp_msg.into()
                }
                #[inline]
                fn prefix(&self) -> $crate::ErrPrefix{
                    $pre
                }
                #[inline]
                fn code(&self) -> u16{
                    $code
                }
                #[inline]
                fn http_code(&self) -> http::StatusCode {
                    $status
                }
            }
    };
    ($t: ty [$pre:expr, $code:literal: $status:expr])=>{
            impl $crate::StatusErr for $t {
                #[inline]
                fn prefix(&self) -> $crate::ErrPrefix{
                    $pre
                }
                #[inline]
                fn code(&self) -> u16{
                    $code
                }
                #[inline]
                fn http_code(&self) -> http::StatusCode {
                    $status
                }
            }
    };
    ($t: ty[$pre: expr, $code: literal ] -> $resp_msg: literal)=>{
        $crate::status_error!($t[$pre , $code:$pre.get_status()]->$resp_msg);
    };
    ($t: ty[$pre: expr, $code: literal ])=>{
        $crate::status_error!($t[$pre , $code:$pre.get_status()]);
    };
}

/// 将外部异常类型进行简单封装的宏
#[macro_export]
macro_rules! error_wrapper {
    ($v:vis $wn:ident($t:ty)) => {
        $v struct $wn ($t);

        impl std::fmt::Display for $wn{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <$t as std::fmt::Display>::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $wn {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <$t as std::fmt::Debug>::fmt(&self.0, f)
            }
        }
        impl std::error::Error for $wn{}

        impl std::convert::Into<$t> for $wn{
            fn into(self) -> $t{
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! resp_error_impl {
    ($t:ty) => {
        /// 实现 Resp -error 可以作为RespResult的异常
        impl ::axum_resp_result::RespError for $t {
            fn log_message(&self) ->  std::borrow::Cow<'_, str> {
                $crate::StatusErr::information(self)
            }

            fn resp_message(&self) ->  std::borrow::Cow<'_, str> {
                $crate::StatusErr::respond_msg(self)
            }

            fn http_code(&self) -> http::StatusCode {
                $crate::StatusErr::http_code(self)
            }

            type ExtraMessage = $crate::status_code::StatusCode;

            fn extra_message(&self) -> Self::ExtraMessage {
                $crate::StatusErr::status(self)
            }

            fn resp_message_default() -> Option<std::borrow::Cow<'static, str>> {
                Some("Operate Success".into())
            }

            fn extra_message_default() -> Option<Self::ExtraMessage> {
                Some($crate::status_code::StatusCode::new($crate::ErrPrefix::NO_ERR,0x0000))
            }
        }
    };
}
