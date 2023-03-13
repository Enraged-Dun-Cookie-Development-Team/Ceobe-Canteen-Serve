use db_ops_prelude::{mongo_models::ceobe::user_property::models::UserMobId, mongodb::bson::oid::ObjectId};
use futures::{
    future::{ready, Ready},
    io::Cursor,
};
use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::UploadSource,
};
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
    pub cookie_id: Option<ObjectId>,
}
#[derive(Debug, Clone, Copy)]
pub struct CombIdToCookieIdPlayLoad<'s> {
    pub file_name: &'s str,
}
pub struct CombIdToCookieSource;

impl<'s> UploadPayload for CombIdToCookieIdPlayLoad<'s> {
    type Source = CombIdToCookieSource;

    const DIR: &'static str = "datasource-comb";

    fn obj_name(&self) -> &str { self.file_name }
}

impl UploadSource for CombIdToCookieSource {
    type Error = serde_json::error::Error;
    type Read = Cursor<Vec<u8>>;
    type ReadFuture<'f> = Ready<Result<Self::Read, Self::Error>>;
    type Source<'r> = &'r CombIdToCookieId;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_> {
        ready(serde_json::to_vec(payload).map(Cursor::new))
    }

    fn content_type(
        _payload: &Self::Source<'_>,
    ) -> ceobe_qiniu_upload::mime_guess::Mime {
        "application/json; charset=utf-8".parse().unwrap()
    }
}
