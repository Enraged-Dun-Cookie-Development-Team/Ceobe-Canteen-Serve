mod codegen;
#[macro_use]
mod utils;
mod checker_info;
mod inner_checker_info;
use codegen::check_obj::CheckObj;
use inner_checker_info::InnerCheckerInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

/// check obj 过程宏，根据挂载的结构体构造复合 `Checker` 已经包装的 `Uncheck`
/// 对象以及配套的 [Future](std::future::Future) 对象
///
/// ## 对被挂载的结构体的要求
///
/// - Named 结构体
/// - 不包含任何泛型参数
/// - 结构体每一项(field)的名称为对应 `Checked` 中的对应字段(field)名称
/// - 结构体中的每一项为合法的 `Checker`
/// - 通常情况该结构体名称为 `XXXChecker` ,该名称将用于生成用于实现 `Checker`
///   的空白结构体
///
/// 以下为一个可供使用的结构体
///
/// ```rust
/// use checker::prefabs::no_check::NoCheck;
///
/// pub struct ExampleChecker{
///     bar: NoCheck<i32>,
///     foo: NoCheck<String>
/// }
/// ```
///
/// ## 对 `Checked` 结构体的要求
///
/// - Named 结构体
/// - 有挂载 `#[derive(typedBuilder)]`
/// - 需要通过 `Checker` 获取的域(field)需要与被挂载结构体对应域(field)相同
/// - 不需要通过 `Checker` 获取的域(field)需要通过
///   [TypedBuilder](typed_builder::TypedBuilder)
///   相关配置，使得可以在不提供相关值时仍然可以`build()`
///
/// 以下为一个可供使用的结构体
///
/// ```rust
/// use typed_builder::TypedBuilder;
///
/// #[derive(Debug, TypedBuilder)]
/// pub struct Example{
///     bar: i32,
///     foo: String,
///     #[builder(default = 11)]
///     default_bar: i32,
///     #[builder(default)]
///     default_foo :Option<String>,
/// }
/// ```
///
/// ## 过程宏参数
///
/// 当以上的相关结构体都准备完毕了，现在就可以开始挂载过程宏了
/// 过程宏需要3个参数，分别是
/// 1. `uncheck` 传递一个标识符，用于作为生成的 `UnCheck` 的名称
/// 2. `checked` 传递一个类型，为`Checked` 结构体的类型
/// 3. `error` 传递一个类型， 为生成的 `Checker` 的异常统一映射目标类型，
/// 要求所有的内部`Checker` 的 `Err` 都实现了 [Into](Into)
///
/// 传递顺序不可打乱，使用`,` 分割，最后的`,` 可选
///
/// 以下为一次可能的挂载过程宏参数
///
/// ```rust
/// 
/// use checker::prefabs::no_check::NoCheck;
/// use typed_builder::TypedBuilder;
///
/// #[checker::check_obj(
///     uncheck = ExampleUncheck,
///     checked = Example,
///     error = std::convert::Infallible
/// )]
/// #[derive(Debug)]
/// pub struct ExampleChecker{
///     bar: NoCheck<i32>,
///     foo: NoCheck<String>,
/// }
///
/// #[derive(Debug, TypedBuilder)]
/// pub struct Example{
///     bar: i32,
///     foo: String,
///     #[builder(default = 11)]
///     default_bar: i32,
///     #[builder(default)]
///     default_foo :Option<String>,
/// }
/// ```
/// ### 注意事项
///
/// - 如果需要挂载额外的过程宏或者 `derive` 宏到 `Uncheck` 结构体上，可以在
///   [check_obj](macro@check_obj)
/// 之下挂载宏，[check_obj](macro@check_obj)将会将其搬运到 `Uncheck` 结构体上
/// - 生成的 `Uncheck` 与 `Checker` 的可见性 与 被挂载结构体的可见性 (`pub`)
///   一致
/// - 生成的 `Uncheck` 中每一个域(field)的可见性 与
///   被挂载结构体的对应域(field) 一致
/// - 如果要在 `Uncheck`
///   的某一域(field)上挂载过程宏等，
///   可直接在被挂载对象对应的域(field)上进行挂载， [check_obj](macro@check_obj)
///   将会搬运到相应的位置
///
/// ## 生成什么？
///
/// 1. `Uncheck` 结构体，名称通过过程宏参数提供
/// 2. `Checker` 空白结构体，并为其实现
/// `Checker`
/// 3. `CheckerFut`
/// [!Unpin](std::marker::Unpin) 的结构体，并为其实现
/// [Future](std::future::Future) ，在其内部实现具体`check`过程
#[proc_macro_attribute]
pub fn check_obj(params: TokenStream, item: TokenStream) -> TokenStream {
    let params = parse_macro_input!(params as checker_info::CheckerInfo);
    let body = parse_macro_input!(item as ItemStruct);

    let body = syn_error!(InnerCheckerInfo::from_item_struct(body));

    let check_obj = CheckObj::from((params, body));

    quote::quote! {#check_obj}.into()
}
