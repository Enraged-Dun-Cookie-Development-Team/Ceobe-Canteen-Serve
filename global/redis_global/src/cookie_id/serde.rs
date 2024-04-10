use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for super::CookieId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for super::CookieId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?
            .parse()
            .map_err(|err| serde::de::Error::custom(err))?;
        Ok(Self(str))
    }
}
