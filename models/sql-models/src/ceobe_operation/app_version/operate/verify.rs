use crate::ceobe_operation::app_version::models::model_app_version;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use super::AppVerionCounts;
use super::{CeobeOperationAppVersionSqlOperate, OperateResult};

impl CeobeOperationAppVersionSqlOperate {
    pub async fn is_exist_app_version(version: String, db: &impl ConnectionTrait,) -> OperateResult<bool> {
        let resp = model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version))
            .select_only()
            .column_as(model_app_version::Column::Id.count(), "count")
            .into_model::<AppVerionCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }
}
