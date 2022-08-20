use jwt::VerifyWithKey;
use time_usage::sync_time_usage_with_name;

use super::{get_key, User};

pub fn decrypt_token(token_str: String) -> Result<User, jwt::Error> {
    sync_time_usage_with_name("解析JWT Token", || {
        let key = get_key();
        let user_info = token_str.verify_with_key(key)?;
        Ok(user_info)
    })
}
