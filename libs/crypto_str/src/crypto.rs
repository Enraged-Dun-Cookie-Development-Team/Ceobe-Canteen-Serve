use std::{borrow::Cow, cmp::PartialEq};

use crate::Encoder;

pub enum CryptoString<'p, E> {
    Raw(Cow<'p, str>, E),
    Crypto(Cow<'p, str>),
}

impl<'p, E: Encoder> CryptoString<'p, E> {
    pub fn new_raw<S>(raw: S) -> Self
    where
        E: Default,
        S: Into<String>,
    {
        Self::Raw(Cow::Owned(raw.into()), E::default())
    }

    pub fn new_crypto<S>(raw: S) -> Self
    where
        S: Into<String>,
    {
        Self::Crypto(Cow::Owned(raw.into()))
    }

    pub fn crypto(self) -> Result<Self, E::Error> {
        match self {
            CryptoString::Raw(r, _) => E::encode(r).and_then(|e| Ok(Self::Crypto(e))),
            c => Ok(c),
        }
    }

    pub fn verify(&self, rhs: &Self) -> std::result::Result<bool, E::Error> {
        match (self, rhs) {
            (Self::Raw(r, _), Self::Raw(r2, _)) => Ok(r == r2),
            (Self::Raw(r, _), Self::Crypto(c)) => E::verify(c, r),
            (Self::Crypto(c), Self::Raw(r, _)) => E::verify(c, r),
            (Self::Crypto(c1), Self::Crypto(c2)) => Ok(c1 == c2),
        }
    }
}
