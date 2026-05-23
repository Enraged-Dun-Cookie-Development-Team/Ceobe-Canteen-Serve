use http::StatusCode;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorType {
    pub mark: char,
    pub ident: String,
    pub description: String,
    #[serde(
        alias = "defaultStatus",
        deserialize_with = "deserialize_status_code"
    )]
    pub default_status_code: StatusCode,
    pub error: Vec<Error>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    pub ident: String,
    #[serde(
        rename = "statusCode",
        deserialize_with = "deserialize_option_status_code",
        default
    )]
    pub http_code: Option<StatusCode>,
    pub description: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorCfg {
    pub kind: Vec<ErrorType>,
}

impl ErrorCfg {
    /// 验证配置合法性：同 `[[kind]]` 内无重复 ident
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for kind in &self.kind {
            // 检查同一 kind 内 error ident 是否重复
            let mut seen = std::collections::HashSet::new();
            for err in &kind.error {
                if !seen.insert(&err.ident) {
                    errors.push(format!(
                        "Kind `{}` (mark: {}): 重复的 error ident `{}`",
                        kind.ident, kind.mark, err.ident
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        }
        else {
            Err(errors)
        }
    }
}

fn deserialize_status_code<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<StatusCode, D::Error> {
    let code = <u16 as Deserialize>::deserialize(d)?;
    StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}

fn deserialize_option_status_code<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<StatusCode>, D::Error> {
    let code = <Option<u16> as Deserialize>::deserialize(d)?;
    code.map(|code| {
        StatusCode::from_u16(code).map_err(serde::de::Error::custom)
    })
    .transpose()
}

#[cfg(test)]
mod test {
    use crate::payloads::ErrorCfg;

    #[test]
    fn test_load() {
        let v = include_str!("../../.././example_error_config.toml");
        let payload: ErrorCfg = toml::from_str(v).expect("Error");

        println!("{payload:?}")
    }
}
