use std::any::Any;

use axum::{
    body::Body as BoxBody, extract::OriginalUri, response::IntoResponse,
};
use axum_resp_result::RespResult;
use http::Method;
use status_err::generated_error::{
    no_err_kind::NotAnError as GenNotAnError,
    not_found_kind::RouteNotExistError as GenRouteNotExistError,
    serve_kind::ServicePanicError as GenServicePanicError,
};
use tracing::{error, instrument, warn};

#[macro_export]
/// 1. 辅助构造枚举形式的Error, 并提供 [Form](std::convert::Form)转换实现， 和
///    [StatusErr](status_err::StatusErr)实现
///
/// ```rust
///     error_generate!(
///     //      |------- 构造的枚举型异常的类型名称
///     pub GolbalError
///     // |--------------枚举类型的名称
///     // |     |-------每一枚举类型内部的类型
///       Io=std::io::Error  // 多个内部类型用空格区分
///       Db=sea_orm::DbErr
///     );
/// ```
/// 2. 为现有类型生成包装类型
/// ```rust
///     error_generate!(
///     //   |------------新建包装类型的可见性
///     //   |     |------新建包装类型的类型名称
///         pub JsonError
///         (      
///             Error  // 内部包装的类型
///         )"反序列化异常" // 为包装类型添加额外的异常信息
///     );
/// ```
macro_rules! error_generate {
    ($v:vis $err_name:ident $($v_name:ident=$inner_err:ty)+ ) => {
        #[derive(Debug, status_err::ThisError, status_err::StatusErr)]
        #[status_err(resp_err)]
        // TODO: 后续修复 large_enum_variant，考虑 Box 包装大变体
        #[allow(clippy::large_enum_variant)]
        $v enum $err_name{
            $(
                #[error(transparent)]
                #[status_err(err = "transparent")]
                $v_name(#[from] $inner_err)
            ),+
        }

        impl From< ::core::convert::Infallible > for $err_name {
            fn from(_: ::core::convert::Infallible)->Self{
                unreachable!()
            }
        }

    };
    ($v:vis $err_name:ident = $msg:literal)=>{
        #[derive(Debug)]
        $v struct $err_name;
        impl std::error::Error for $err_name{}
        impl std::fmt::Display for $err_name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{} => {}",stringify!($err_name), $msg)
            }
        }

        impl serde::Serialize for $err_name{
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                serializer.serialize_str($msg)
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
                writeln!(f, "{} => {} `{}`",stringify!($wrap_name), $msg, self.0)
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

status_err::status_error!(
    new pub RouteNotExistError["该路由不存在，请检查请求路径"]
    => GenRouteNotExistError
);
status_err::resp_error_impl!(RouteNotExistError);

status_err::status_error!(
    new pub ServicePanic["服务器发生未预期的异常"]
    => GenServicePanicError
);
status_err::resp_error_impl!(ServicePanic);

status_err::status_error!(
    new pub NotAnError[""]
    => GenNotAnError
);
status_err::resp_error_impl!(NotAnError);

#[instrument(name = "router-not-found")]
pub async fn not_exist(
    OriginalUri(uri): OriginalUri, method: Method,
) -> RespResult<(), RouteNotExistError> {
    warn!(
        route.exist = false,
        request.uri = ?uri,
        request.method = ?method
    );
    RespResult::err(RouteNotExistError)
}

#[instrument(skip_all)]
#[deprecated(note = "暂时没用使用")]
pub fn serve_panic(
    error: Box<dyn Any + Send + 'static>,
) -> http::Response<BoxBody> {
    let detail = if let Some(msg) = error.downcast_ref::<String>() {
        msg.as_str()
    }
    else if let Some(msg) = error.downcast_ref::<&str>() {
        *msg
    }
    else {
        "Unknown panic message"
    };

    error!(unexpectedPanic.detail = detail);
    RespResult::<(), _>::err(ServicePanic).into_response()
}
