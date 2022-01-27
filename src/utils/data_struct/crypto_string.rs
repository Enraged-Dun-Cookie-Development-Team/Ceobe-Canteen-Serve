use super::{range_limit::LimitError, RangeLimitString};
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

const FIX_SIZE:usize=64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CryptoString<const L: usize, const H: usize> {
    Raw(RangeLimitString<L, H>),
    Cryptoed(String),
}

impl<'s, const L: usize, const H: usize> Into<&'s str> for &'s CryptoString<L, H> {
    fn into(self) -> &'s str {
        match self {
            CryptoString::Raw(raw) => raw.as_ref().as_str(),
            CryptoString::Cryptoed(s) => s.as_str(),
        }
    }
}

impl<const L: usize, const H: usize> Into<String> for CryptoString<L, H> {
    fn into(self) -> String {
        match self {
            CryptoString::Raw(raw) => raw.into(),
            CryptoString::Cryptoed(arr) => arr,
        }
    }
}

impl<const L: usize, const H: usize> AsRef<str> for CryptoString<L, H> {
    fn as_ref(&self) -> &str {
        self.into()
    }
}

impl<const L: usize, const H: usize> TryFrom<String> for CryptoString<L, H> {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != FIX_SIZE {
            Err("Size Not Match".to_string())
        } else {
            Ok(Self::Cryptoed(value))
        }
    }
}

impl<const L: usize, const H: usize> CryptoString<L, H> {
    pub fn new_raw<T: ToString>(data: T) -> Result<CryptoString<L, H>, LimitError> {
        let data = data.to_string();
        Ok(Self::Raw(RangeLimitString::try_from(data)?))
    }
    fn crypto(raw: &str) -> String {
        let mut hasher = Sha3::keccak256();
        hasher.input_str(&raw);
        hasher.result_str()
    }
    pub fn into_crypto(self) -> Self {
        let res = match self {
            CryptoString::Raw(raw) => {
                let res = Self::crypto(&raw);
                res
            }
            c => return c,
        };

        Self::Cryptoed(res)
    }
}

impl<const L: usize, const H: usize> Serialize for CryptoString<L, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CryptoString::Raw(r) => {
                let st = Self::crypto(&r);
                st.serialize(serializer)
            }
            CryptoString::Cryptoed(st) => st.serialize(serializer),
        }
    }
}

impl<'de, const L: usize, const H: usize> Deserialize<'de> for CryptoString<L, H> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(res) => {
                if res.len() == FIX_SIZE {
                    Ok(Self::Cryptoed(res))
                } else {
                    Ok(Self::Raw(
                        RangeLimitString::<L, H>::try_from(res)
                            .or_else(|e| Err(serde::de::Error::custom(e)))?,
                    ))
                }
            }
            Err(err) => Err(err),
        }
    }
}
