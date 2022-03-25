use jwt::VerifyWithKey;
use super::set_token;

fn decrpyt_token(token_str: String) -> Result<set_token::User, jwt::Error> {
    let key = set_token::GLOBAL.get();
    let user_info = token_str.verify_with_key(key)?;
    Ok(user_info)
}
