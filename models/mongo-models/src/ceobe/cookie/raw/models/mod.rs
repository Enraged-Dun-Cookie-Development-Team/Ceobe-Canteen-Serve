use mongodb::bson::{oid::ObjectId, Binary};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sub_model::SubModel;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct RawModel {
    pub source_type: String,
    pub data_id: String,
    pub source_config_id: i32,
    pub ty: String,
    pub raw_cookie: RawCookie,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieId {
    pub _id: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct RawCookie {
    pub content: Binary,
    pub extra: Value,
}
