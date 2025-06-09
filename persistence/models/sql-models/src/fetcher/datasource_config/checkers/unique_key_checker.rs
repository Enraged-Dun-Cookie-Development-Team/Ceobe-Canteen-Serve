use checker::{
    prefabs::str_len_checker::StrMaxCharLenChecker, Checker, RefChecker,
};
use futures::future::{ready, Ready};
use serde_json::{Map, Value};
use typed_builder::TypedBuilder;
use url::Url;

use super::DatasourceUnique;

#[derive(Debug, TypedBuilder)]
pub struct PreCheckFetcherDatasourceConfig {
    id: Option<i32>,
    platform: String,
    datasource: String,
    nickname: String,
    avatar: Url,
    unique_key: Option<String>,
    config: Map<String, Value>,
    jump_url: Option<Url>,
    visual: bool
}

#[derive(Debug, TypedBuilder)]
pub struct FetcherDatasourceConfig {
    pub id: Option<i32>,
    pub platform: String,
    pub datasource: String,
    pub nickname: String,
    pub avatar: Url,
    pub unique_key: DatasourceUnique,
    pub config: Map<String, Value>,
    pub jump_url: Option<Url>,
    pub visual: bool,
}

pub struct UniqueKeyChecker;

impl Checker for UniqueKeyChecker {
    type Args = ();
    type Checked = FetcherDatasourceConfig;
    type Err = super::CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = PreCheckFetcherDatasourceConfig;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(
            'checker: {
                // if not provide unique key using 0
                let Some(unique_key) = uncheck.unique_key
                else {
                    break 'checker Ok("-");
                };

                // try get the unique for number
                let Some(Value::String(identify)) =
                    uncheck.config.get(&unique_key)
                else {
                    break 'checker Err(super::CheckError::UniqueKeyInvalid(
                        unique_key,
                    ));
                };

                if let Err(err) =
                    StrMaxCharLenChecker::<_, 64>::ref_checker((), identify)
                        .into_inner()
                {
                    break 'checker Err(err.into());
                }

                Ok(identify.as_str())
            }
            .map(ToOwned::to_owned)
            .map(|unique| {
                FetcherDatasourceConfig::builder()
                    .id(uncheck.id)
                    .avatar(uncheck.avatar)
                    .config(uncheck.config)
                    .datasource(uncheck.datasource)
                    .platform(uncheck.platform)
                    .nickname(uncheck.nickname)
                    .unique_key(unique)
                    .jump_url(uncheck.jump_url)
                    .visual(uncheck.visual)
                    .build()
            }),
        )
    }
}
