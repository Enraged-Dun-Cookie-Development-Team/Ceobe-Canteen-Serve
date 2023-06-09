use db_ops_prelude::mongo_models::ceobe::user_property::models::UserMobId;
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

/// 返回数据源列表与组合id
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
pub struct DatasourceConfig {
    pub datasource_config: Vec<Uuid>,
    pub datasource_comb_id: String,
}

/// 返回数据源组合id
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DatasourceCombResp {
    pub datasource_comb_id: String,
}
