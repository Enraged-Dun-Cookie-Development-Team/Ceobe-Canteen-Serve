use std::borrow::Cow;

use crate::{Crypto, CryptoString, CryptoWarp, Encoder, Raw};

impl<E> From<String> for CryptoWarp<Raw, E>
where
    E: Encoder + Default,
{
    fn from(s: String) -> Self {
        CryptoWarp(Raw, CryptoString::new_raw(s))
    }
}

impl<E> From<String> for CryptoWarp<Crypto, E>
where
    E: Encoder + Default,
{
    fn from(s: String) -> Self {
        CryptoWarp(Crypto, CryptoString::new_crypto(s))
    }
}

impl<'s, E> TryInto<Cow<'s, str>> for &'s CryptoWarp<Crypto, E>
where
    E: Encoder,
{
    type Error = E::Error;

    fn try_into(self) -> Result<Cow<'s, str>, Self::Error> {
        match &self.1 {
            CryptoString::Raw(r, _) => E::encode(Cow::Borrowed(r)),
            CryptoString::Crypto(c) => Ok(Cow::Borrowed(&c)),
        }
    }
}

