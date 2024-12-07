use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{FetchOptionViewValue, OptionViewField, _private::SealTrait};

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

impl<T> FetchOptionViewValue<T> for SkipField {
    fn fetch_option(self) -> Option<T> { None }
}

impl SealTrait for SkipField {}
