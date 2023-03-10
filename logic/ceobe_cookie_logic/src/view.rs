use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;
use serde_json::{Value, Map};


// 分页饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListResp {
    pub cookies: Vec<Value>,
    pub next_page_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListReq {
    pub datasource_comb_id: String,
    pub cookie_id: String,
}