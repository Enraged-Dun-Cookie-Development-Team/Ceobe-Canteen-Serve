use checker::{
    check_obj,
    prefabs::{
        no_check::NoCheck, option_checker::OptionChecker,
        str_len_checker::StrMaxCharLenChecker, url_checker::UrlChecker,
    },
};
use sea_orm::{IntoActiveModel, Set};
use sql_connection::ext_traits::active_or_set::ActiveOrSet;
use typed_builder::TypedBuilder;
use url::Url;

use crate::ceobe_operation::tool_link::{
    checkers::CheckError, models::model_tool_link::ActiveModel,
};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationToolLink {
    pub id: Option<i32>,
    pub nickname: String,
    pub avatar: Url,
    pub jump_url: Url,
}
#[check_obj(
    uncheck = CeobeOperationToolLinkUncheck,
    checked = CeobeOperationToolLink,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct PreCheckCeobeOperationToolLinkChecker {
    pub id: OptionChecker<NoCheck<i32>>,
    pub nickname: StrMaxCharLenChecker<String, 32>,
    pub avatar: UrlChecker,
    pub jump_url: UrlChecker,
}

impl IntoActiveModel<ActiveModel> for CeobeOperationToolLink {
    /// CeobeOperationToolLink
    ///
    /// id:
    ///     - 如果id为null，则为新增，uuid新建
    ///     - 如果id有值，这更改，uuid为空
    fn into_active_model(self) -> ActiveModel {
        let mut active = ActiveModel {
            nickname: Set(self.nickname),
            avatar: Set(self.avatar.to_string()),
            jump_url: Set(self.jump_url.to_string()),
            ..Default::default()
        };
        active.id.set_optional(self.id);

        active
    }
}
