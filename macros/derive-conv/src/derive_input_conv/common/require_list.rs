use std::marker::PhantomData;

use crate::derive_input_conv::common::list::List;

/// 对于每一个 require item 项，都需要以下内容
///
/// - var_name: 需要的变量（来自preprocess）或者field(来自Self/Model)的名称
/// - require_mode: 需要的变量的模式，包括以下几种
///     - `Ref`(default): 默认情况为取得原始值的不可变引用
///     - `RefMut`: 取得对应变量的不可变引用
///     - `Owned`: 取得所有权（对应的field 会被标记为 `ignore`)
///     - `Copy`: 使用Copy 取得所有权，（对应的field 不会被标记为 `ignore`）
// TODO 完成参数需求项解析
pub struct RequireItem<Mode = FullMode> {
    _phantom: PhantomData<Mode>,
}

pub type RequireList<Mode = FullMode> = List<RequireItem<Mode>>;

pub struct FullMode;

pub struct RefOnlyMode;
/// 生成转换代码的标记
/// 需要实现 [FromMeta](darling::FromMeta)
///
/// 将对应输入的字面量转换为对应的枚举项。如
/// - `"mut"` -> `Self::RefMut`,
/// - `"ref"` -> `Self::Ref`,
/// - `"owned"` -> `Self::Owned`,
/// - `"copy"` -> `Self::Copy`,
///
/// 如果无法转换为任意的枚举项，报错
pub enum RequireMode {
    Ref,
    RefMut,
    Owned,
    Copy,
}
