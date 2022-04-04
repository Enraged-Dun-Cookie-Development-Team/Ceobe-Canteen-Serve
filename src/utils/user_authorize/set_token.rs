use jwt::SignWithKey;
use time_usage::sync_time_usage_with_name;

use super::{get_key, User};

pub trait GenerateToken {
    fn generate(self) -> Result<String, jwt::Error>;
}

impl GenerateToken for User {
    fn generate(self) -> Result<String, jwt::Error> {
        sync_time_usage_with_name("生成JWT Token", || {
            let key = get_key();
            let token_str = self.sign_with_key(key)?;
            Ok(token_str)
        })
    }
}
