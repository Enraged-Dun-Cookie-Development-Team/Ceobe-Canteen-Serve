mod meta;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::{ Value};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use self::meta::Meta;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct AnalyzeModel {
    pub meta: Meta,
    pub raw_id: ObjectId,
    pub source_config_id: i32,
    pub text: String,
    pub status: CookieAnalyzeStatus,
    pub images: Option<Vec<String>>,
    pub tags: Value,
    pub keywords: Value
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