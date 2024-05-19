use checker::{
    check_obj,
    prefabs::{
        no_check::NoCheck, option_checker::OptionChecker,
        str_len_checker::StrMaxCharLenChecker, url_checker::UrlChecker,
    },
    ToCheckRequire,
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
    pub slogen: String,
    pub description: String,
    pub tags: String,
}
#[check_obj(
    uncheck = CeobeOperationToolLinkUncheck,
    checked = CeobeOperationToolLink,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize, TypedBuilder)]
pub struct PreCheckCeobeOperationToolLinkChecker {
    #[builder(setter(
        transform = |id:Option<i32>| ToCheckRequire::require_check(id),
    ))]
    pub id: OptionChecker<NoCheck<i32>>,
    #[builder(setter(
        transform = |nickname:String| ToCheckRequire::require_check(nickname)
    ))]
    pub nickname: StrMaxCharLenChecker<String, 32>,
    #[builder(setter(
        transform = |avatar:String| ToCheckRequire::require_check(avatar)
    ))]
    pub avatar: UrlChecker,
    #[builder(setter(
        transform = |jump_url:String| ToCheckRequire::require_check(jump_url)
    ))]
    pub jump_url: UrlChecker,
    #[builder(setter(
        transform = |slogen:String| ToCheckRequire::require_check(slogen)
    ))]
    pub slogen: StrMaxCharLenChecker<String, 16>,
    #[builder(setter(
        transform = |description:String| ToCheckRequire::require_check(description)
    ))]
    pub description: StrMaxCharLenChecker<String, 64>,
    #[builder(setter(
        transform = |tags:String| ToCheckRequire::require_check(tags)
    ))]
    pub tags: StrMaxCharLenChecker<String, 64>,
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
            slogen: Set(self.slogen),
            description: Set(self.description),
            tags: Set(self.tags),
            ..Default::default()
        };
        active.id.set_optional(self.id);

        active
    }
}
