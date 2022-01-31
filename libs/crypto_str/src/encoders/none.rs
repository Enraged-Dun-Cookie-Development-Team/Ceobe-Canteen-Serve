use std::borrow::Cow;

use crate::Encoder;

pub struct NoCrypto;

impl Encoder for NoCrypto {
    type Error = NoErr;

    fn encode<'s>(raw: Cow<'s,str>) -> Result<std::borrow::Cow<'s, str>, Self::Error> {
        Ok(raw)
    }

    fn verify<'s, S: AsRef<str>>(
        cryptoed: &std::borrow::Cow<'s, str>,
        input: &S,
    ) -> Result<bool, Self::Error> {
        Ok(cryptoed == input.as_ref())
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
