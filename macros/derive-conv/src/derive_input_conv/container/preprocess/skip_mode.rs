/// 与粗砺过程可以在生成某些模式中不执行，通过该参数指定
/// 需要实现 [FromMeta](darling::FromMeta)
///
/// ## 输入格式
/// mode 名称为 [`ConvMode`](super::super::conv_mode::ConvMode) 的对应字面量，
/// 如果需要在多个模式下不执行，使用 `|` 进行分割，顺序无关。以下为可能的输入
/// - `"IntoActiveModel"`
/// - `"UpdateActiveModel|FromModel"`
///
/// ## Note
///
/// 不能同时跳过全部的生成模式。比如，以下输入非法
/// - `UpdateActiveModel|FromModel|IntoActiveModel`

pub struct SkipMode {
    // TODO 完成预处理过程跳过模式 参数解析
}
