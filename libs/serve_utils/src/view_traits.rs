pub mod const_field;
mod option_field;
mod skip_field;
mod value_field;

/// 用于view中的可选field
///
/// 由于view可能存面向不同端有细微差别，
/// 但是如果由于少量的字段区分重复整个view定义会带来大量代码冗余。
/// 因此，提出了该trait
///
/// ## Example
/// 如果有某个view结构体，
/// - 其在后台接口的view需要提供其ID
/// - 在前台接口的view中不需要提供ID
/// - 在请求接口中会基于是否提供ID进行不同操作
///
/// 那么使用[OptionViewField]就可以如下实现
///
/// ```rust
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct FooView<ID:OptionViewField<i32>>{
///     id:ID,
///     foo:String
/// }
/// ```
///
/// - 当在后台接口中使用时，其为 `FooView<ValueField<i32>>`,
///   即需要提供id的view。可能的序列化结果为
/// ```json
/// {
///     "id": 114514,
///     "foo": "hello"
/// }
/// ```
/// - 当在前台接口中使用时，其为 `FooView<SkipField>`,
///   即不携带id的view。可能的序列化结果为
/// ```json
/// {
///     "foo": "hello"
/// }
/// ```
/// - 当需要根据是否提供ID执行不同操作实现，其为
///   `FooView<OptionValueField<i32>>`, 即携带Option的ID的view
pub trait OptionViewField<V>: _private::SealTrait {
    fn skip_serde(&self) -> bool;
}

pub trait FetchViewValue<T> {
    fn fetch(self) -> T;
}

pub trait FetchOptionViewValue<T> {
    fn fetch_option(self) -> Option<T>;
}

mod _private {
    pub trait SealTrait {}
}

pub use option_field::OptionField;
pub use skip_field::SkipField;
pub use value_field::ValueField;
