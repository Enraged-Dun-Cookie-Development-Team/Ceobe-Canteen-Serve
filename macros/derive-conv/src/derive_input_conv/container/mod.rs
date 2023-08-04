

mod conv_mode;
mod preprocess;
mod generate_fields;

/// 容器的参数解析
/// 
/// 需要实现 [FromMeta](darling::FromMeta)
/// 
/// ## 需要参数
/// - `mode` : 需要生成的模块，可以提供一个或者多个，类型为 [ConvMode](conv_mode::ConvMode)
/// - `target`: 生成的目标Model, SQL 的 Model，类型为 [`Type`](syn::Type)
/// - `preprocess`: 预处理过程，类型为 [`Preprocess`](preprocess::Preprocess),注意预处理过程可能有0个或者多个
/// - `non_exist_use_default`（optional）: `IntoActiveModel` 专用参数，未提供的值使用 [`Default::default`]填充，
/// 类型为 [bool], 
/// - `generate_fields`(optional): 额外生成的field, 类型为 [`GenerateFields`](generate_fields::GenerateFields)
// TODO 完成容器的参数解析
pub struct ContainerDeriveInput{
}