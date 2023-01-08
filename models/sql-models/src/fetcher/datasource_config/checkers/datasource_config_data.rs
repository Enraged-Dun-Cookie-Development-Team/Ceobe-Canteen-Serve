use checker::{
    check_obj,
    prefabs::{
        no_check::NoCheck, option_checker::OptionChecker,
        str_len_checker::StrMaxCharLenChecker, url_checker::UrlChecker,
    },
};
use sea_orm::{IntoActiveModel, Set};
use serde_json::{Map, Value};
use sql_connection::ext_traits::active_or_set::ActiveOrSet;
use tracing_unwrap::ResultExt;
use typed_builder::TypedBuilder;
use url::Url;
use uuid::Uuid;

use super::CheckError;
use crate::fetcher::datasource_config::models::model_datasource_config::ActiveModel;

#[derive(Debug, TypedBuilder)]
pub struct FetcherDatasourceConfig {
    pub id: Option<i32>,
    pub platform: String,
    pub datasource: String,
    pub nickname: String,
    pub avatar: Url,
    pub config: Map<String, Value>,
}

#[check_obj(
    uncheck = FetcherDatasourceConfigUncheck,
    checked = FetcherDatasourceConfig,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct FetcherDatasourceConfigChecker {
    pub id: OptionChecker<NoCheck<i32>>,
    pub platform: StrMaxCharLenChecker<String, 64>,
    pub datasource: StrMaxCharLenChecker<String, 64>,
    pub nickname: StrMaxCharLenChecker<String, 32>,
    pub avatar: UrlChecker,
    pub config: NoCheck<Map<String, Value>>,
}

impl IntoActiveModel<ActiveModel> for FetcherDatasourceConfig {
    /// FetcherDatasourceConfig转ActiveModel
    ///
    /// id:
    ///     - 如果id为null，则为新增，uuid新建
    ///     - 如果id有值，这更改，uuid为空
    fn into_active_model(self) -> ActiveModel {
        let mut active = ActiveModel {
            nickname: Set(self.nickname),
            avatar: Set(self.avatar.to_string()),
            config: Set(serde_json::to_string(&self.config)
                .expect_or_log("config为非法json格式")),
            ..Default::default()
        };
        active.id.set_optional(self.id);

        if self.id.is_none() {
            active.platform = Set(self.platform);
            active.datasource = Set(self.datasource);
            active.unique_id = Set(Uuid::new_v4())
        }

        active
    }
}
