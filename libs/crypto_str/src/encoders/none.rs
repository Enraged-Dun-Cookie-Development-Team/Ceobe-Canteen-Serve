use std::borrow::Cow;

use crate::Encoder;

pub struct NoCrypto;

impl Encoder for NoCrypto {
    type Error = NoErr;

    fn encode(
        raw: Cow<'_, str>,
    ) -> Result<std::borrow::Cow<'_, str>, Self::Error> {
        Ok(raw)
    }

    fn verify< S: AsRef<str>>(
        encrypted: &str, input: &S,
    ) -> Result<bool, Self::Error> {
        Ok(encrypted == input.as_ref())
    }
}

#[derive(Debug)]
pub struct NoErr;

impl std::error::Error for NoErr {}
impl std::fmt::Display for NoErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "No Error")
    }
}
