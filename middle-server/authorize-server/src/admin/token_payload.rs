use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use persistence::admin;
use serde::{Deserialize, Serialize};
use crate::admin::configure::LocalAuthConfig;
use crate::token_conv::JwtTokenConv;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UserClaim {
    pub(crate) id: i32,
    pub(crate) password_version: u32,
    #[serde(rename = "exp")]
    expiration_time: usize,
    #[serde(rename = "iat")]
    issue_time: usize,
}

impl UserClaim {
    pub fn new(user_id: i32, password_version: u32) -> Self {
        let issue_time = Local::now().timestamp() as usize;
        Self {
            id: user_id,
            password_version,
            issue_time,
            expiration_time: issue_time + 3600 * 24 * 365 * 2,
        }
    }

    pub fn from_model(model: admin::models::Model) -> Self {
        Self::new(model.id, model.num_pwd_change)
    }
}

impl JwtTokenConv for UserClaim {
    fn from_jwt_token(
        payload: &str,
    ) -> jsonwebtoken::errors::Result<Self> {
        let key = LocalAuthConfig::decoder_key();
        let mut validation = Validation::new(Algorithm::HS384);
        validation.set_required_spec_claims(&["iat", "exp"]);
        let payload = decode::<Self>(payload, key, &validation)?;
        Ok(payload.claims)
    }

    fn to_jwt_token(
        &self,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let key = LocalAuthConfig::encoder_key();
        let header = Header::new(Algorithm::HS384);
        encode(&header, self, key)
    }
}

#[cfg(test)]
mod test {
    use crate::admin::configure::{AuthConfig, LocalAuthConfig};
    use crate::admin::token_payload::UserClaim;
    use crate::token_conv::JwtTokenConv;

    struct TestConfig;

    impl AuthConfig for TestConfig {
        fn jwt_key(&self) -> &[u8] { b"abcdefghijklmn" }
    }
    #[test]
    fn test_generate_jwt() {
        LocalAuthConfig::set(&TestConfig);

        let user = UserClaim::new(1, 1);

        let token = user.to_jwt_token().expect("TO JWT Error");

        println!("Token is : {token}")
    }

    #[test]
    fn test_decode_jwt() {
        LocalAuthConfig::set(&TestConfig);
        let user = UserClaim::new(1, 1);

        let token = user.to_jwt_token().expect("TO JWT Error");
        let payload =
            UserClaim::from_jwt_token(&token).expect("Decoder Error");

        assert_eq!(payload, user)
    }
}
