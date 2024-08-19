use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::view_traits::_private::SealTrait;

pub trait OptionViewField: _private::SealTrait {
    fn skip_serde(&self) -> bool;
}

mod _private {
    pub trait SealTrait {}
}

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
        let _ = <() as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self)
    }
}

impl OptionViewField for SkipField {
    fn skip_serde(&self) -> bool { true }
}

impl SealTrait for SkipField {}

#[derive(Debug, Clone)]
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

impl<T> OptionViewField for ValueField<T> {
    fn skip_serde(&self) -> bool { false }
}

#[derive(Debug, Clone)]
pub struct OptionValueField<T>(pub Option<T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OptionValueField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value =
            <Option<T> as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

impl<T: Serialize> Serialize for OptionValueField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(&self.0, serializer)
    }
}

impl<T> OptionViewField for OptionValueField<T> {
    fn skip_serde(&self) -> bool { self.0.is_none() }
}

impl<T> SealTrait for OptionValueField<T> {}
