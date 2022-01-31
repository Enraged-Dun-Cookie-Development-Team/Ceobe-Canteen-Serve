use std::{borrow::Cow, ops::Deref};

use crate::Encoder;

#[derive(Debug, Clone)]
pub enum CryptoString<E> {
    Raw(Cow<'static, str>, E),
    Crypto(Cow<'static, str>),
}

impl<E> CryptoString<E>
where
    E: Encoder,
{
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

impl<E> AsRef<str> for CryptoString<E> {
    fn as_ref(&self) -> &str {
        match self {
            CryptoString::Raw(r, _) | CryptoString::Crypto(r) => &r,
        }
    }
}

impl<E> Deref for CryptoString<E> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[cfg(feature = "wrap")]
impl<E> CryptoString<E> {
    pub fn into_crypto(self) -> crate::CryptoWarp<crate::Crypto, E> {
        crate::CryptoWarp(crate::Crypto, self)
    }
}
