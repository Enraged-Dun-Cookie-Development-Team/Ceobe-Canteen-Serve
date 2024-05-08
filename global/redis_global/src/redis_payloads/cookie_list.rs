use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::CookieId;

/// 数据源组合id-最新饼id 上传对象储存
#[derive(Serialize, Deserialize, Clone, Debug, TypedBuilder)]
pub struct CombIdToCookieIdRep {
    #[builder(default, setter(into, strip_option))]
    /// 最新饼id
    pub cookie_id: Option<CookieId>,

    #[builder(default, setter(into, strip_option))]
    /// 后更新的饼id
    pub update_cookie_id: Option<CookieId>,
}

impl CombIdToCookieIdRep {
    pub fn new(
        cookie_id: Option<impl Into<CookieId>>,
        update_cookie_id: Option<impl Into<CookieId>>,
    ) -> Self {
        Self {
            cookie_id: cookie_id.map(Into::into),
            update_cookie_id: update_cookie_id.map(Into::into),
        }
    }
}
