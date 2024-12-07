use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{
    FetchOptionViewValue, FetchViewValue, OptionViewField,
    _private::SealTrait,
};

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct ValueField<T>(pub T);

impl<T> FetchViewValue<T> for ValueField<T> {
    fn fetch(self) -> T { self.0 }
}

impl<T> FetchOptionViewValue<T> for ValueField<T> {
    fn fetch_option(self) -> Option<T> { Some(self.0) }
}

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
