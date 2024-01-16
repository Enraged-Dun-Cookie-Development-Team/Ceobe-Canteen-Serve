use hmac::digest::InvalidLength;
use crate::validation::SECRET_KEY_LENGTH;
#[derive(Debug,thiserror::Error)]
pub enum AuthorizeInitError{
    #[error(transparent)]
    InvalidHttpHeaderName(#[from]http::header::InvalidHeaderName),
    #[error(transparent)]
    InvalidLength(#[from]InvalidLength),
    #[error("expect secret key length {SECRET_KEY_LENGTH} but get {0}")]
    SecretKeyTooShort(usize)
}