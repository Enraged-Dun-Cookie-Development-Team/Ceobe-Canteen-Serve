use hmac::Hmac;
use sha2::Sha256;

mod configures;
mod jwt_parser;

pub type Secret = Hmac<Sha256>;
pub const SECRET_KEY_LENGTH :usize = 32;