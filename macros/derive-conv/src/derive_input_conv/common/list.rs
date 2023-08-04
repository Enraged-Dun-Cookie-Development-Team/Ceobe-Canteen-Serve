/// 解析输入参数中类型相同的列表项
/// 需要实现 [FromMeta](darling::FromMeta)
///
/// 当 输入 [`Meta`](syn::Meta) 类型为 [`Meta::List`](syn::Meta::List)，
/// 将每个列表元素解析为对应Item
pub struct List<Item>(pub Vec<Item>);
