pub use crypto::CryptoString;
pub use encoders::Encoder;
#[cfg(feature = "wrap")]
pub use wrap::{Crypto, CryptoWarp, Raw};

mod crypto;
mod encoders;
#[cfg(feature = "serde")] mod serde;
#[cfg(feature = "trans")] mod trans;
#[cfg(feature = "wrap")] mod wrap;

pub mod inner_encoders {
    #[cfg(feature = "bcrypt")]
    pub mod bcrypt {
        pub use bcrypt_::BcryptError;

        pub use crate::encoders::bcrypt::BcryptEncoder;

        pub type DefaultBcryptEncoder = BcryptEncoder<12>;
        pub type BcryptString = crate::CryptoString<DefaultBcryptEncoder>;
    }

    #[cfg(feature = "none")]
    pub mod none {
        pub use crate::encoders::none::NoCrypto;

        pub type NoCrtpyoString = crate::CryptoString<NoCrypto>;
    }
}
