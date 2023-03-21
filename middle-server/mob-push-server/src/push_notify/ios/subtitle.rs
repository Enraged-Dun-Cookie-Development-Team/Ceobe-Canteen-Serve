use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]
pub struct Subtitle(pub String);

impl From<&str> for Subtitle {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
impl From<String> for Subtitle {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl NotifySerialize for Subtitle {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        struct_serialize.serialize_field("subtitle", &self.0)
    }
}
