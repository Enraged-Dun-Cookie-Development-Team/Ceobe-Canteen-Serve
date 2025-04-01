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

#[cfg(test)]
mod test {
    use crate::utils::{
        mob_verify::MobIdConfig,
        user_authorize::{
            config::AuthConfig, set_auth_config, GenerateToken, User,
        },
    };

    struct TestConfig;

    impl MobIdConfig for TestConfig {}

    impl AuthConfig for TestConfig {
        fn jwt_key(&self) -> &[u8] { b"abcdefghijklmn" }
    }

    #[test]
    fn test_generate_jwt() {
        set_auth_config(&TestConfig);
        let user = User {
            id: 1,
            num_pwd_change: 1,
        };

        let jwt = user.generate().unwrap();

        println!("Token is : {}", jwt)
    }
}
