use persistence::admin;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder,
)]
pub struct User {
    #[serde(rename = "id")]
    user_id: i32,
    #[serde(rename = "num_pwd_change")]
    password_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct VerifiedAuthorize {
    user_id: i32,
    user_name: String,
    password: String,
}

pub type AuthorizeInfo = admin::models::Model;
