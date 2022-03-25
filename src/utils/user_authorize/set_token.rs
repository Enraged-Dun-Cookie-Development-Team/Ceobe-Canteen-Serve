use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

extern crate lazy_static;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    password: String,
}

lazy_static! {
    pub static ref GLOBAL: state::Storage<Hmac<Sha256>> = state::Storage::new();
}

pub trait GenerateToken {
    fn generate(self) -> Result<String, jwt::Error>;
}

impl GenerateToken for User {
    fn generate(self) -> Result<String, jwt::Error> {
        let key = GLOBAL.get_or_set(|| {
            Hmac::new_from_slice(b"fsaDF#Fwe!wefafaf23rEDY%wf").expect("密钥生成失败")
        });
        let token_str = self.sign_with_key(key)?;
        Ok(token_str)
    }
}
