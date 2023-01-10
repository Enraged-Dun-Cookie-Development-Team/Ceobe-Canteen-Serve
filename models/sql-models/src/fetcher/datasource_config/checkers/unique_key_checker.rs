use checker::Checker;
use futures::future::{ready, Ready};
use serde_json::Map;
use typed_builder::TypedBuilder;
use url::Url;

use super::DatasourceUnique;

#[derive(Debug, TypedBuilder)]
pub struct PreCheckFetcherDatasourceConfig {
    pub id: Option<i32>,
    pub platform: String,
    pub datasource: String,
    pub nickname: String,
    pub avatar: Url,
    pub unique_key: Option<String>,
    pub config: Map<String, serde_json::Value>,
}

#[derive(Debug, TypedBuilder)]
pub struct FetcherDatasourceConfig {
    pub id: Option<i32>,
    pub platform: String,
    pub datasource: String,
    pub nickname: String,
    pub avatar: Url,
    pub unique_key: DatasourceUnique,
    pub config: Map<String, serde_json::Value>,
}

pub struct UniqueKeyChecker;

impl Checker for UniqueKeyChecker {
    type Args = ();
    type Checked = FetcherDatasourceConfig;
    type Err = super::CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = PreCheckFetcherDatasourceConfig;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready('checker: {
            // if not provide unique key using 0
            let Some(unique_key) = uncheck.unique_key else{
                break 'checker Ok(DatasourceUnique::from(0));
            };

            // try get the unique for number
            let Some(serde_json::Value::Number(number))= uncheck.config.get(&unique_key)else{
                break 'checker Err(super::CheckError::UniqueKeyInValid(unique_key))
            };
            // get u64
            let Some(identify) = number.as_u64() else{
                break 'checker Err(super::CheckError::UniqueKeyInValid(unique_key))
            };

            Ok(
                DatasourceUnique::from(identify)
                )
        }.map(|unique|{
            FetcherDatasourceConfig::builder()
            .id(uncheck.id)
            .avatar(uncheck.avatar)
            .config(uncheck.config)
            .datasource(uncheck.datasource)
            .platform(uncheck.platform)
            .nickname(uncheck.nickname).unique_key(unique)
            .build()
        }))
    }
}
