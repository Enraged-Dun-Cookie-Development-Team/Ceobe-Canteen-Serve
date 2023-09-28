use hmac::Hmac;
use sha2::Sha256;

mod configures;
pub type Secret = Hmac<Sha256>;
