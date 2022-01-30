mod crypto;
mod encoders;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "trans")]
mod trans;
#[cfg(feature = "wrap")]
mod wrap;

pub use encoders::Encoder;
pub mod inner_encoders {
    #[cfg(feature = "bcrypt")]
    pub mod bcrypt {
        pub use crate::encoders::bcrypt::BcryptEncoder;
        pub type DefaultBcryptEncoder = BcryptEncoder<12>;
        pub type BcryptString = crate::CryptoString<DefaultBcryptEncoder>;
    }

    #[cfg(feature = "none")]
    pub mod none {
        pub use crate::encoders::none::{NoCrypto, NoErr};
        pub type NoCrtpyoString = crate::CryptoString<NoCrypto>;
    }
}

pub use crypto::CryptoString;

#[cfg(feature = "wrap")]
pub use wrap::{Crypto, CryptoWarp, Raw};
