pub mod meta;

use std::default;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::{ Value};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use self::meta::Meta;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    vis = "pub",
    name = "CookieInfo",
    extra(derive(Debug, Clone, Serialize, Deserialize, TypedBuilder))
))]
pub struct AnalyzeModel {
    pub meta: Meta,
    #[sub_model(ignore("CookieInfo"))]
    pub raw_id: ObjectId,
    pub source_config_id: i32,
    pub text: String,
    #[sub_model(ignore("CookieInfo"))]
    pub status: CookieAnalyzeStatus,
    pub images: Option<Vec<String>>,
    pub compress_images: Option<Vec<Option<String>>>,
    pub tags: Option<Value>,
    #[sub_model(ignore("CookieInfo"))]
    pub keywords: Option<Value>
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieId {
    pub _id: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CookieAnalyzeStatus {
    #[serde(rename = "ANALYZE_WITHOUT_IO_SUCCESS", alias = "ANALYZE_WITHOUT_IO_SUCCESS")]
    AnalyzeWithoutIoSuccess,
    #[serde(rename = "ANALYZE_SUCCESS", alias = "ANALYZE_SUCCESS")]
    AnalyzeSuccess
}