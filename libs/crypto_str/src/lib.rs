mod crypto;
mod encoders;
#[cfg(feature = "serde")]
mod serde;
mod trans;

mod wrap;

pub use encoders::Encoder;
pub mod inner_encoders {
    #[cfg(feature = "bcrypt")]
    pub mod bcrypt {
        pub use crate::encoders::bcrypt::BcryptEncoder;
        pub type DefaultBcryptEncoder = BcryptEncoder<12>;
        pub type BcryptString = crate::CryptoString<DefaultBcryptEncoder>;
    }
}

pub use crypto::CryptoString;
pub use wrap::{Crypto, CryptoWarp, Raw};
