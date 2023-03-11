use ceobe_qiniu_upload::{JsonPayload, PayloadLocal};
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

/// MobId请求
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, Default)]
pub struct DatasourceConfig {
    pub datasource_config: Vec<Uuid>,
    pub datasource_comb_id: String,
}


/// 数据源组合id-最新饼id 上传对象储存
#[derive(Debug, Clone, Serialize)]
pub struct CombIdToCookieId {
    pub cookid_id: Option<String>,
}
pub struct CombIdToCookieIdPlayLoad{
    pub cookie_id: Option<String>,
    pub file_name: String
}

impl JsonPayload for CombIdToCookieIdPlayLoad {
    type Payload = CombIdToCookieId;

    fn payload(self) -> Self::Payload {
        CombIdToCookieId {
            cookid_id: self.cookie_id,
        }
    }
}

impl PayloadLocal for CombIdToCookieIdPlayLoad {
    fn obj_name(&self) -> &str {
        &self.file_name
    }
}


