/// 条件赋值/更新
/// 需要实现 [FromMeta](darling::FromMeta)
/// 当符合指定条件时，会赋值，否则不赋值
///
/// ## 需求参数
/// - `condition`（optional）: 判定是否进行赋值的函数，类型为
///   [Callable](super::super::common::callable::Callable),
/// 该函数的第一个参数为该field本身，转移所有权，后续参数为 `requires`
/// 按顺序进入，只能持有 ref 引用， 返回值为 `Option<Ty>`, `Ty` 为当前field
/// 的类型， 当返回结果为 `Some` 时，赋值，否则不赋值
/// - `requires`(optional): 条件赋值判定需要的额外参数列表，只能来自
///   `preprocess` 结果，只能获取 ref 引用，
/// 类型为 [`List<syn::Ident>`](crate::derive_input_conv::common::list::List<syn::Ident>),
/// 或者 [`RequireList`](crate::derive_input_conv::common::require_list::RequireList)
///
/// ## NOTE
/// 如果 field 的类型本身就为Optional,
/// 那可以考虑直接进行条件赋值，也就是 `condition` 与 `Requires` 均不提供
pub struct ConditionSet {
    // TODO: 完成 `ConditionSet` 条件置值的参数解析
}
