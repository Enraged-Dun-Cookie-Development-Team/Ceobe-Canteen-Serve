use std::{borrow::Cow, convert::Infallible};

use crate::Encoder;

pub struct NoCrypto;

impl Encoder for NoCrypto {
    type Error = Infallible;

    fn encode(
        raw: Cow<'_, str>,
    ) -> Result<std::borrow::Cow<'_, str>, Self::Error> {
        Ok(raw)
    }

    fn verify<S: AsRef<str>>(
        encrypted: &str, input: &S,
    ) -> Result<bool, Self::Error> {
        Ok(encrypted == input.as_ref())
    }
}
