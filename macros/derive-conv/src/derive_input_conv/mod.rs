mod common;
mod container;
mod field;

/// 过程宏参数解析
///
/// ## 实现要求
/// 需要实现 [FromDeriveInput](darling::FromDeriveInput)
/// - 限制为Named_struct
/// - 检查不包含任何泛型参数（或者允许部分泛型参数）
/// - 其他检查...
/// ## 需要参数
///
/// - `ident`: 原始结构体本身的标识符， 类型为 [`syn::Ident`]
/// - `container_attr` : 结构体上的输入参数，类型为
///   [`ContainerDeriveInput`](container::ContainerDeriveInput)
/// - `fields`: 结构体的field 参数列表，类型为
///   [`Data<Ignored,field::FieldInput>`](darling::ast::Data)
///
/// ## Note
///
/// - 该宏使用的 attribute 为 `conv`
/// - 该宏只支持 named-struct
/// - 该宏可能支持 泛型参数
// TODO: 完成过程宏输入参数解析
pub struct ConvHelperDeriveInput {}
