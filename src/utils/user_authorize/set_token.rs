use jwt::SignWithKey;

use super::{get_key, User};

pub trait GenerateToken {
    fn generate(self) -> Result<String, jwt::Error>;
}

impl GenerateToken for User {
    fn generate(self) -> Result<String, jwt::Error> {
        let key = get_key();
        let token_str = self.sign_with_key(key)?;
        Ok(token_str)
    }
}
