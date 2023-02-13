use mongo_models::ceobe::user::models::UserMobId;
use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;



/// 至今为止最大存活数量
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MobIdReq {
    pub mob_id: String,
}

impl Into<UserMobId> for MobIdReq {
    fn into(self) -> UserMobId {
        UserMobId { mob_id: self.mob_id }
    }
}