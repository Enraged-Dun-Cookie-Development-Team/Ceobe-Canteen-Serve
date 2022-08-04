use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
};

use super::{
    AppVerionCounts, CeobeOperationAppVersionSqlOperate, OperateResult,
};
use crate::ceobe_operation::app_version::models::model_app_version;

impl CeobeOperationAppVersionSqlOperate {
    pub async fn is_exist_app_version(
        version: String, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
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
