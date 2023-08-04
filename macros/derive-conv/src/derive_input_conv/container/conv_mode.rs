/// 生成转换代码的标记
///
/// 需要实现 [FromMeta](darling::FromMeta)
///
/// 将对应输入的字面量转换为对应的枚举项。如
/// - `"IntoActiveModel"` -> `Self::IntoActiveModel`
///
/// 如果无法转换为任意的枚举项，报错
///
/// ## NOTE
///
/// `FromModel` 与其他 2 者为互斥关系
// todo 完成 ConvMode 参数解析
pub enum ConvMode{
    IntoActiveModel,
    UpdateActiveModel,
    FromModel
}