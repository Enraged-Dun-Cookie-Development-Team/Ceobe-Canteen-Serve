use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;



/// 至今为止最大存活数量
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MobIdReq {
    pub mob_id: String,
}