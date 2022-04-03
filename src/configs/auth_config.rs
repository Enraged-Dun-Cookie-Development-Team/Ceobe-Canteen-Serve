use rand::RngCore;
use serde::Deserialize;

use crate::utils::user_authorize::config;

crate::quick_struct! {
    #[derive(Default)]
    pub AuthConfig{
        #[serde(default="Default::default",alias="jwt-key")]
        jwt:Jwt
        #[serde(alias="header",default="default_token")]
        header_name:String
    }
}

fn default_token() -> String { String::from("Token") }

impl config::AuthConfig for AuthConfig {
    fn jwt_key(&self) -> &[u8] { &self.jwt.0 }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Jwt([u8; 32]);

impl Default for Jwt {
    fn default() -> Self {
        let key: [u8; 32] = rand::random();
        Self(key)
    }
}

impl<'de> Deserialize<'de> for Jwt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut rand = rand::thread_rng();
        let mut idx = 0;
        let mut inner_key = [0u8; 32];
        let inner = String::deserialize(deserializer)?;
        let mut key = inner.bytes();

        while let Some(c) = key.next() {
            inner_key[idx] = c;
            idx += 1;
            if idx == 32 {
                break;
            }
        }
        if idx < 32 {
            rand.fill_bytes(&mut inner_key[idx..32]);
        }

        Ok(Jwt(inner_key))
    }
}
