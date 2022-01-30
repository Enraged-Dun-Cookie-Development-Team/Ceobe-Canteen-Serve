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
