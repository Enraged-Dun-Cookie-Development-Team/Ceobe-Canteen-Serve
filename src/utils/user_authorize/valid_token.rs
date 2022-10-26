use jwt::VerifyWithKey;

use super::{get_key, User};

pub fn decrypt_token(token_str: String) -> Result<User, jwt::Error> {
    let key = get_key();
    let user_info = token_str.verify_with_key(key)?;
    Ok(user_info)
}
