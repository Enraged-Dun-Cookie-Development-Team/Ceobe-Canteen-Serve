use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DeleteOneToolLinkReq {
    pub id: i32,
}
