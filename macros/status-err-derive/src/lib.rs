#[macro_use]
mod utils;
mod code_gen;
mod input_loading;
use darling::FromDeriveInput;
use input_loading::derive_info::StatusErrorDeriveInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

/// Status Err Derive Macro 辅助实现[StatusErr](status_err::StatusErr)
///
/// ## 类型挂载参数
///
/// - `resp_err` 添加这个参数后，该Derive macro 将会自动为挂载的类型实现
///   [RespError](resp_result::RespError)
///
/// ## 枚举值挂载参数
///
/// - `err = "transparent"` 使用这个参数表明本层包装在实现
///   [StatusErr](status_err::StatusErr) 时保持透明。
///   也就是说，在实现时将会直接调用内部的包装类型的
///   [StatusErr](status_err::StatusErr)
///   对应的方法。使用这个参数要求对应的枚举值为元组类型，
///   并且只包含一个内部值。该模式为默认
/// - `err( .. )` 手动指定 [StatusErr](status_err::StatusErr)
///   相关参数，可以使用的参数如下
///     - `resp_msg = "Resp Message"` 响应时使用的异常信息，默认为 [ToString]
///       的结果
///     - `err_code = 0x0013` 响应时使用的异常码，为数字字面量，必须
///     - `prefix = `ErrPrefix::NOT_FOUND`` 响应时状态码前缀，必须
///     - `http_code = "HttpCode::NOT_FOUND"` 特殊指定 Http
///       状态码，默认为前缀默认状态码
///
/// ## Example
///
/// ```rust
/// use status_err::{ErrPrefix, HttpCode, StatusErr};
/// #[derive(Debug, status_err::ThisError, status_err::StatusErr)]
/// #[status_err(resp_err)]
/// pub enum TestErr {
///     #[error("UTF8 编码解析异常 {0}")]
///     Parse(#[from] std::string::FromUtf8Error),
///     #[error("其他异常 {start:?}")]
///     #[status_err(err(
///         resp_msg = "其他异常",
///         err_code = 12,
///         prefix = "ErrPrefix::CHECKER",
///         http_code = "HttpCode::NOT_FOUND"
///     ))]
///     Else { start: String },
/// }
/// ```
#[proc_macro_derive(StatusErr, attributes(status_err))]
pub fn status_error_derive(derive_input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(derive_input as DeriveInput);
    if !derive_input.generics.params.is_empty() {
        return syn::Error::new(
            derive_input.generics.params.span(),
            "Status Error can not using Generic",
        )
        .to_compile_error()
        .into();
    }
    if let Data::Struct(_) | Data::Union(_) = derive_input.data {
        return syn::Error::new(
            derive_input.span(),
            "Status Error can Only using on Enum",
        )
        .to_compile_error()
        .into();
    }

    let status_err = darling_error!(
        StatusErrorDeriveInfo::from_derive_input(&derive_input)
    );

    let token = syn_error!(status_err.checking());

    quote::quote! {
        #token
    }
    .into()
}
