
pub mod skip_mode;

/// 解析预处理内容参数
/// 需要实现 [FromMeta](darling::FromMeta)
///
/// ## 预处理需要以下的参数
/// 
/// - var(optional): 接受预处理结果的表达式。如果预处理过程本身没有结果，可以不提供，
/// - process: 预处理过程本身，为一个 [`Callable`](super::super::common::callable::Callable) 对象。
/// 这个函数的参数列表为 `requires` 按顺序传入，返回值与 `var` 相匹配
/// - requires（optional）: 预处理过程的额外参数，来自`Self`,
/// 如果预处理过程不需要除了 `Self` 以外的任何参数，可以不提供requires
/// - require_self(optional): 预处理过程需要 `Self`,可以通过提供值来获得 `&mut Self` 或者 `Copy Self`,
/// 否则默认为 `& Self` ，以下为可能的参数传递方式
///     - `require_self`: 获得 `&Self`
///     - `require_self = "mut"` 获得 `&Self`
///     - `require_self = "copy"` 获得 `Self` (Self 本身所有权转移语义为 COPY)
/// 
/// ## NOTE
/// 
/// 请注意 `requires` 与 `require_self` 是互斥的参数,即同时只能提供一个参数，如果同时提供需要报错。
// TODO: 完成预处理流程的参数解析
pub struct Preprocess{
}

