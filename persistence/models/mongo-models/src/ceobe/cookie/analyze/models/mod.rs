use images::CookieImages;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use self::meta::Meta;

pub mod images;
pub mod meta;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(
    all(
        vis = "pub",
        name = "CookieInfo",
        extra(derive(Debug, Clone, Serialize, Deserialize, TypedBuilder))
    ),
    none(
        vis = "pub",
        name = "CookieSimpleInfo",
        extra(derive(Debug, Clone, Serialize, Deserialize, TypedBuilder))
    )
)]
pub struct AnalyzeModel {
    #[sub_model(want("CookieSimpleInfo"))]
    pub meta: Meta,
    #[sub_model(ignore("CookieInfo"))]
    pub raw_id: ObjectId,
    pub source_config_id: i32,
    #[sub_model(want("CookieSimpleInfo"))]
    pub text: String,
    #[sub_model(ignore("CookieInfo"))]
    pub status: CookieAnalyzeStatus,
    #[sub_model(want("CookieSimpleInfo"))]
    pub images: Option<Vec<CookieImages>>,
    pub tags: Option<Value>,
    #[sub_model(ignore("CookieInfo"))]
    pub keywords: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieId {
    pub _id: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieInfoWithId {
    pub _id: ObjectId,
    #[serde(flatten)]
    pub cookie_info: CookieInfo,
}

// 泰拉记事社漫画数量与最终更新时间
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct TerraComicAggregate {
    /// 漫画id
    pub comic: String,
    /// 漫画最后更新时间
    pub update_time: i64,
    /// 该漫画总数量
    pub count: i32,
}

// 泰拉记事社漫画小章节信息
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct TerraComicEpisodeInfo {
    /// 漫画id
    pub comic: String,
    /// 跳转链接
    pub jump_url: String,
    /// 短标题
    pub short_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CookieAnalyzeStatus {
    AnalyzeWithoutIoSuccess,
    AnalyzeSuccess,
}
