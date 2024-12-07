use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{OptionViewField, _private::SealTrait};

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
