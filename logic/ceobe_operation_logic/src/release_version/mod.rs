use persistence::{
    mongodb::MongoDatabaseOperate,
    mysql::sea_orm::prelude::async_trait::async_trait,
    operate::{FromRef, FromRequestParts, Parts},
};
use tencent_cloud_server::{
    axum_starter::{PartTencentCloudManagerState, RequestClient},
    cloud_manager::TencentCloudManager,
};

pub struct ReleaseVersionLogic {
    pub mongodb: MongoDatabaseOperate,
    pub tencent_cloud: TencentCloudManager,
}

#[async_trait]
impl<S> FromRequestParts<S> for ReleaseVersionLogic
where
    S: Send + Sync + 'static,
    RequestClient: FromRef<S>,
    PartTencentCloudManagerState: FromRef<S>,
{
    type Rejection = Rejection;

    async fn from_request_parts(
        parts: &mut Parts, state: &S,
    ) -> Result<Self, Self::Rejection> {
        let mongodb = <MongoDatabaseOperate as FromRequestParts<S>>::from_request_parts(parts, state).await?;
        let tencent_cloud =
            <TencentCloudManager as FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await?;
        Ok(Self {
            mongodb,
            tencent_cloud,
        })
    }
}

mod error;
mod implement;
pub mod view;

use self::error::{Rejection,LogicResult};
use self::view::*;
pub use self::error::Error;
