use actix_web::HttpRequest;
use resp_result::RespResult;
use status_err::ErrPrefix;

use crate::database::error::DatabaseError;

#[macro_export]
/// 1. 辅助构造枚举形式的Error,  
/// 并提供 [Form](std::form::Form)转换实现，
/// 和 [StatusErr](status_err::StatusErr)实现
///     ```rust
///     error_generate!(
///             // |------- 构造的枚举型异常的类型名称
///         pub GolbalError
///       // |--------------枚举类型的名称
///       // |     |-------每一枚举类型内部的类型
///         Io=std::io::Error  // 多个内部类型用空格区分
///         Db=sea_orm::DbErr
///     );
///     ```
///2. 为现有类型生成包装类型
///     ```rust
///         error_generate!(
///         //   |------------新建包装类型的可见性
///         //   |     |------新建包装类型的类型名称
///             pub JsonError
///             (      
///                 actix_web::Error  // 内部包装的类型
///             )"反序列化异常" // 为包装类型添加额外的异常信息
///         );
///     ```
macro_rules! error_generate {
    ($v:vis $err_name:ident $($v_name:ident=$inner_err:ty)+ ) => {
        #[derive(Debug)]
        $v enum $err_name{
            Infallible,
            $(
                $v_name($inner_err)
            ),+
        }
        impl std::error::Error for $err_name{}
        impl std::fmt::Display for $err_name{
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self{
                    $(
                        Self::$v_name(err)=>{write!(f, "{} Error : {}",stringify!($v_name), err)}
                    ),+
                    Self::Infallible=>unreachable!(),
                }
            }
        }
        /// 实现 status Error 可供下一级封装使用
        impl status_err::StatusErr for $err_name{
            #[inline]
            fn prefix(&self) -> status_err::ErrPrefix{
                match self{
                    $(
                        Self::$v_name(err)=>{err.prefix()}
                    ),+
                    Self::Infallible=>unreachable!(),
                }
            }
            #[inline]
            fn code(&self) -> u16{
                match self{
                    $(
                        Self::$v_name(err)=>{err.code()}
                    ),+
                    Self::Infallible=>unreachable!(),
                }
            }

            #[inline]
            fn http_code(&self)->http::StatusCode{
                match self{
                    $(
                        Self::$v_name(err)=>{err.http_code()}
                    ),+
                    Self::Infallible=>unreachable!(),
                }
            }
        }
        // 实现 Resp -error 可以作为RespResult的异常
        status_err::resp_error_impl!($err_name);
        // 转换代码
        $(
            impl From<$inner_err> for $err_name{
                #[inline]
                fn from(src: $inner_err) -> Self {
                    Self::$v_name(src)
                }
            }

        )+

        impl From<std::convert::Infallible> for $err_name{
            #[inline]
            fn from(_: std::convert::Infallible) -> Self {
                Self::Infallible
            }
        }
    };
    ($v:vis $err_name:ident = $msg:literal)=>{
        #[derive(Debug)]
        $v struct $err_name;
        impl std::error::Error for $err_name{}
        impl std::fmt::Display for $err_name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               writeln!(f, "{} Error : {}",stringify!($err_name), $msg)
            }
        }
    };

    ($v:vis $wrap_name:ident($err_ty:ty))=>{
        $crate::error_generate!($v $wrap_name($err_ty)"");
    };

    ($v:vis $wrap_name:ident($err_ty:ty)$msg:literal)=>{
        #[derive(Debug)]
        $v struct $wrap_name($err_ty);
        impl std::error::Error for $wrap_name{}
        impl std::fmt::Display for $wrap_name{
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               writeln!(f, "{} Error : {}`{}`",stringify!($err_name), $msg, self.0)
            }
        }
        impl From<$err_ty> for $wrap_name{
            #[inline]
            fn from(src:$err_ty)->Self{
                Self(src)
            }
        }
    };

}

error_generate!(
    pub GlobalError
    Io=std::io::Error
    Db=DatabaseError
    Route=RouteNotExistError
);

status_err::status_error! {
    pub RouteNotExistError[
        ErrPrefix::NOT_FOUND,
        0002
    ]=>"该路由不存在，请检查请求路径"
}

status_err::resp_error_impl!(RouteNotExistError);

pub async fn not_exist(req: HttpRequest) -> RespResult<(), RouteNotExistError> {
    log::info!("路由未找到 `{}` {}", req.path(), &req.method());
    RespResult::err(RouteNotExistError)
}
