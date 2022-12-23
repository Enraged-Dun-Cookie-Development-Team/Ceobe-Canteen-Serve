use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::fetcher::platform_config::models::model_platform_config;
use super::{FetcherPlatformConfigSqlOperate, PlatformCounts, OperateResult};



impl FetcherPlatformConfigSqlOperate {
    // 查询时候存在type_id的平台
    pub async fn is_platform_exist(
        db: &impl ConnectionTrait, type_id: &str
    ) -> OperateResult<bool> {
        let resp = model_platform_config::Entity::find()
            .filter(model_platform_config::Column::TypeId.eq(type_id))
            .select_only()
            .column_as(model_platform_config::Column::Id.count(), "count")
            .into_model::<PlatformCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }
}