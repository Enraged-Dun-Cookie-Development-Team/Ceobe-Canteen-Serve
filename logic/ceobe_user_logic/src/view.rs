use mongo_models::ceobe::user::models::UserMobId;
use serde::{Serialize, Deserialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;



/// MobId请求
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MobIdReq {
    pub mob_id: String,
}

impl Into<UserMobId> for MobIdReq {
    fn into(self) -> UserMobId {
        UserMobId { mob_id: self.mob_id }
    }
}

/// MobId请求
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatasourceConfig {
    pub datasource_config: Vec<Uuid>,
}

impl DatasourceConfig {
    pub fn new() -> Self {
        DatasourceConfig {
            datasource_config: Vec::new(),
        }
    }
}