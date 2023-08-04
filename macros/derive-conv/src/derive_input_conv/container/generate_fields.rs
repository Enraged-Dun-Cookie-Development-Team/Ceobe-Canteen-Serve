use crate::derive_input_conv::common::list::List;

/// 为转换提供额外的field
/// 需要实现 [FromMeta](darling::FromMeta)
/// 
/// - 如果为 `IntoActiveModel` 模式，会添加额外的 `Set` 项，
/// - 如果为 `UpdateActiveModel` 模式，会更新这些额外项，
/// - 如果为 `FromModel` 模式，忽略该参数
/// 
/// 需要以下内容
/// - field_name: 额外添加的field 的名称，
/// - from_var（optional）: field 的值的来处，只能来自于 preprocess 结果。
/// 如果该field 未提供，那么就与 field_name 相同
// TODO: 完成额外Field 参数解析
pub struct GenerateField{
    
}

pub type GenerateFields = List<GenerateField>;