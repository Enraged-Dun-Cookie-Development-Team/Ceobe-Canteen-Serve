use std::borrow::Cow;

use crate::Encoder;
#[derive(Default, Debug, Clone)]
pub struct BcryptEncoder<const COST: u32>;

impl<const C: u32> Encoder for BcryptEncoder<C> {
    type Error = bcrypt_::BcryptError;

    fn encode<'s>(raw: Cow<'s, str>) -> Result<std::borrow::Cow<'s, str>, Self::Error> {
        bcrypt_::hash(raw.as_ref().as_bytes(), C).and_then(|s| Ok(Cow::Owned(s)))
    }

    fn verify<'s, S: AsRef<str>>(
        cryptoed: &std::borrow::Cow<'s, str>,
        input: &S,
    ) -> Result<bool, Self::Error> {
        bcrypt_::verify(input.as_ref(), &*cryptoed)
    }
}

#[cfg(test)]
mod test_bcrypt {
    use crate::{encoders::Encoder, inner_encoders::bcrypt::DefaultBcryptEncoder};

    #[test]
    fn test_match() {
        let pwd = "123456";
        let encode_pwd = DefaultBcryptEncoder::encode(pwd.into()).unwrap();

        println!("encode pwd: {}", encode_pwd);
        assert!(encode_pwd.len() < 64);
        assert!(DefaultBcryptEncoder::verify(&encode_pwd, &"123456").unwrap());
    }
}
