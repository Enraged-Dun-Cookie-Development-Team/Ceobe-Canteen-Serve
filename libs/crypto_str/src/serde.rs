use crate::{
    crypto::CryptoString,
    wrap::{Crypto, CryptoWarp, Raw},
    Encoder,
};

impl<'de, E> serde_::Deserialize<'de> for CryptoWarp<Raw, E>
where
    E: Encoder + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(Raw, CryptoString::new_raw(s)))
    }
}

impl<'de, E> serde_::Deserialize<'de> for CryptoWarp<Crypto, E>
where
    E: Encoder + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(Crypto, CryptoString::new_crypto(s)))
    }
}

impl<E> serde_::Serialize for CryptoString<E>
where
    E: Encoder,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_::Serializer,
    {
        match self {
            CryptoString::Raw(r, _) | CryptoString::Crypto(r) => serializer.serialize_str(r),
        }
    }
}
