use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

// 分页饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListResp {
    pub cookies: Vec<Value>,
    pub next_page_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(deny_unknown_fields)]
pub struct CookieListReq {
    pub datasource_comb_id: String,
    pub cookie_id: String,
}
