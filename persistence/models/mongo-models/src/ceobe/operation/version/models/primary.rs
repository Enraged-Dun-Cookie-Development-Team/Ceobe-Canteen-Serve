use serde::{
    de::Unexpected, Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Primary;

impl<'de> Deserialize<'de> for Primary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s =
            <Option<&str> as Deserialize<'de>>::deserialize(deserializer)?;
        match s {
            Some("primary") | None => Ok(Primary),
            Some(s) => {
                Err(serde::de::Error::invalid_value(
                    Unexpected::Str(s),
                    &"primary",
                ))
            }
        }
    }
}

impl Serialize for Primary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("primary")
    }
}

pub trait SkipPrimarySerialize {
    fn should_skip(&self) -> bool;
}

impl SkipPrimarySerialize for Primary {
    fn should_skip(&self) -> bool { true }
}

impl SkipPrimarySerialize for String {
    fn should_skip(&self) -> bool { false }
}
