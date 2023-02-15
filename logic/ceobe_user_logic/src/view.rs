use mongo_models::ceobe::user::models::UserMobId;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

/// MobId请求
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MobIdReq {
    pub mob_id: String,
}

impl From<MobIdReq> for UserMobId {
    fn from(mob_id: MobIdReq) -> Self {
        UserMobId {
            mob_id: mob_id.mob_id,
        }
    }
}

/// MobId请求
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
pub struct DatasourceConfig {
    pub datasource_config: Vec<Uuid>,
}
