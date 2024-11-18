use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::view_traits::_private::SealTrait;

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

mod _private {
    pub trait SealTrait {}
}

/// View 中可选field的可以跳过情况
#[derive(Default, Debug, Clone, Copy)]
pub struct SkipField;

impl Serialize for SkipField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for SkipField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = <Option<()> as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self)
    }
}

impl<V> OptionViewField<V> for SkipField {
    fn skip_serde(&self) -> bool { true }
}

impl SealTrait for SkipField {}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct ValueField<T>(pub T);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for ValueField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <T as Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

impl<T: Serialize> Serialize for ValueField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <T as Serialize>::serialize(&self.0, serializer)
    }
}

impl<T> SealTrait for ValueField<T> {}

impl<T> OptionViewField<T> for ValueField<T> {
    fn skip_serde(&self) -> bool { false }
}

#[derive(Debug, Clone)]
pub struct OptionField<T>(pub Option<T>);

impl<T> Default for OptionField<T> {
    fn default() -> Self { Self(None) }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OptionField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value =
            <Option<T> as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

impl<T: Serialize> Serialize for OptionField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(&self.0, serializer)
    }
}

impl<T> OptionViewField<T> for OptionField<T> {
    fn skip_serde(&self) -> bool { self.0.is_none() }
}

impl<T> SealTrait for OptionField<T> {}
