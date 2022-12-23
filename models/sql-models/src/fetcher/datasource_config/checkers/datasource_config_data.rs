
use checker::{check_obj, prefabs::{option_checker::OptionChecker, url_checker::UrlChecker, no_check::NoCheck}};
use range_limit::{RangeBoundLimit, limits::max_limit::MaxLimit};
use sea_orm::{Unset, ActiveValue::NotSet, Set};
use typed_builder::TypedBuilder;
use url::Url;
use uuid::Uuid;

use super::CheckError;
use crate::fetcher::datasource_config::models::model_datasource_config;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[derive(Debug, TypedBuilder)]
pub struct FetcherDatasourceConfig {
    pub id: Option<i32>,
    pub platform: String, 
    pub datasource: String,
    pub nickname: String,
    pub avatar: Url,
    pub config: String,
}

#[check_obj(
    uncheck = FetcherDatasourceConfigUncheck,
    checked = FetcherDatasourceConfig,
    error = CheckError
)]
#[derive(Debug,serde::Deserialize)]
pub struct FetcherDatasourceConfigChecker {
    pub id: OptionChecker<NoCheck<i32>>,
    pub platform: MaxLimitString<64>, 
    pub datasource: MaxLimitString<64>,
    pub nickname: MaxLimitString<16>,
    pub avatar: UrlChecker,
    pub config: NoCheck<String>,
}

impl model_datasource_config::ActiveModel {
    /// FetcherDatasourceConfig转ActiveModel
    /// 
    /// id:
    ///     - 如果id为null，则为新增，uuid新建
    ///     - 如果id有值，这更改，uuid为空
    pub(in crate::fetcher::datasource_config) fn datasource_config_into_active_model(
        FetcherDatasourceConfig {
            id,
            platform,
            datasource,
            nickname,
            avatar,
            config,
        }: FetcherDatasourceConfig,
    ) -> Self {
        Self {
            id: match id {
                Some(id) => Set(id),
                None => NotSet,
            },
            platform: match id {
                Some(_) => NotSet,
                None => Set(platform),
            },
            datasource: match id {
                Some(_) => NotSet,
                None => Set(datasource),
            },
            nickname: Set(nickname),
            avatar: Set(avatar.to_string()),
            config: Set(config),
            unique_id: match id {
                Some(_) => NotSet,
                None => Set(Uuid::new_v4()),
            },
        }
    }
}
