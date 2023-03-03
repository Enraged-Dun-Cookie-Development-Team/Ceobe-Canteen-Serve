use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;
use mongo_migration::mongo_models::ceobe::user_property::models::UserMobId;
use resp_result::{Nil, RespResult};

use self::error::MobVerifyError;

mod error;
mod layer;
mod service;

pub struct MobIdInfo(pub UserMobId);

impl std::fmt::Debug for MobIdInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MobIdInfo")
            .field("mob_id", &self.0.mob_id)
            .finish()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MobIdInfo
where
    S: Send + Sync,
{
    type Rejection = RespResult<Nil, MobVerifyError>;

    async fn from_request_parts(
        parts: &mut Parts, _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts
            .extensions
            .remove::<Self>()
            .ok_or(MobVerifyError::NoMobIdLayer)
            .map_err(RespResult::err)?)
    }
}
