mod condition_set;
/// 结构体Field 参数解析
/// 需要实现 [FromField](darling::FromField)
///
/// ## 需要内容
/// - 结构体field 本身
///     - ident: field 原始的标识符
///     - ty: filed 原始的类型
/// - Attr 参数
///     - `ignore`(optional): 转换时忽略该field, 当模式为 `FromModel`
///       时无效,类型为 [bool]
///     - `condition_set`(optional): 当符合条件时，才进行赋值/更新，
/// 当模式为 `FromModel` 时无效, 类型为
/// [ConditionSet](condition_set::ConditionSet)
///     - `rename`(optional): 将该值赋值/修改到 rename 的 field 中，或者 从
///       rename 的 field 处取得值,
/// 类型为 [String]
///     - `project`（optional）:  对结果作用一个类型转换
///         - 当为 `IntoActiveModel` 或者 `UpdateActiveModel`
///           模式时，为将当前field 映射到 ActiveModel 中对应的类型，
///         - 当为 `FromModel` 时，为将model 中对应的Field 映射到当前field
///           的类型
///         - 当该field 被某个 `preprocess` Owned require 时，不作用 任何
///           projection
///         - 类型为 [Callable](super::common::callable::Callable),一个入参，
///           一个返回值
///
/// ## NOTE
///
/// 1. `ignore` 参数与其他的 attr 参数互斥
/// 2. `project` 会在 `condition_set` 后进行，即 `condition_set`
///    的参数本身是原始的值与类型
// TODO 完成Field 输入参数解析
pub struct FieldInput {}
