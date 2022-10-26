use std::borrow::Cow;

use resp_result::{ConfigTrait, RespConfig, SignType, StatusSign};

#[derive(Debug, serde::Deserialize)]
pub struct RespResultConfig {
    // serde configs
    #[serde(alias = "body", alias = "b")]
    body_name: String,
    #[serde(alias = "err", alias = "err-msg")]
    err_msg_name: String,
    #[serde(alias = "fix-field", default = "Default::default")]
    full_field: bool,
    #[serde(alias = "bool-status")]
    signed_status: Option<SignError>,
    #[serde(alias = "body-extra-err")]
    body_extra_err: Option<String>,
    // resp configs
    #[serde(alias = "header-extra-err")]
    header_extra_err: Option<String>,
}

impl ConfigTrait for RespResultConfig {}

impl resp_result::SerdeConfig for RespResultConfig {
    fn body_name(&self) -> Cow<'static, str> { self.body_name.clone().into() }

    fn err_msg_name(&self) -> Cow<'static, str> {
        self.err_msg_name.clone().into()
    }

    fn fixed_field(&self) -> bool { self.full_field }

    fn signed_status(&self) -> Option<StatusSign> {
        self.signed_status
            .as_ref()
            .cloned()
            .map(|v| {
                match v {
                    SignError::Simple(key) => (key, SignType::Bool),
                    SignError::BoolRev { key, rev } => {
                        (
                            key,
                            if rev {
                                SignType::BoolRevert
                            }
                            else {
                                SignType::Bool
                            },
                        )
                    }
                    SignError::Num { key, ok, fail } => {
                        (key, SignType::new_number(ok, fail))
                    }
                    SignError::Str { key, ok, fail } => {
                        (key, SignType::new_str(ok, fail))
                    }
                }
            })
            .map(|(k, v)| StatusSign::new(k, v))
    }

    fn extra_message(&self) -> Option<Cow<'static, str>> {
        self.body_extra_err.clone().map(Into::into)
    }
}

impl RespConfig for RespResultConfig {
    fn head_extra_code(&self) -> Option<Cow<'static, str>> {
        self.header_extra_err.clone().map(Into::into)
    }
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone, Eq)]
#[serde(untagged)]
pub enum SignError {
    Simple(String),
    BoolRev {
        key: String,
        #[serde(default)]
        rev: bool,
    },
    Num {
        key: String,
        ok: u8,
        fail: u8,
    },
    Str {
        key: String,
        ok: String,
        fail: String,
    },
}
#[cfg(test)]
mod test {
    use super::SignError;

    #[test]
    fn test_128() {
        let a = 128u8;
        println!("{a:b}");
    }
    #[test]
    fn test_serde() {
        // basic name only
        let json = serde_json::json! {
            "Acvv"
        };
        let v =
            serde_json::from_value::<super::SignError>(json).expect("Bad");
        assert_eq!(v, SignError::Simple(String::from("Acvv")));
        // name with rev
        let json = serde_json::json! {
            {
                "key": "Acvv",
                "rev": true
            }
        };
        let v =
            serde_json::from_value::<super::SignError>(json).expect("Bad");
        assert_eq!(
            v,
            SignError::BoolRev {
                key: String::from("Acvv"),
                rev: true
            }
        );

        // name with ok/fail code u8
        let json = serde_json::json! {
            {
                "key": "Acvv",
                "ok": 200,
                "fail":100
            }
        };
        let v =
            serde_json::from_value::<super::SignError>(json).expect("Bad");
        assert_eq!(
            v,
            SignError::Num {
                key: String::from("Acvv"),
                ok: 200,
                fail: 100
            }
        );

        // name with ok/fail code u8
        let json = serde_json::json! {
            {
                "key": "Acvv",
                "ok": "ok",
                "fail":"failure"
            }
        };
        let v =
            serde_json::from_value::<super::SignError>(json).expect("Bad");
        assert_eq!(
            v,
            SignError::Str {
                key: String::from("Acvv"),
                ok: String::from("ok"),
                fail: String::from("failure")
            }
        );
    }
}
