use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect,
};
use tap::{Pipe, Tap};
use tracing::info;

use super::{
    AppVersionCounts, CeobeOperationAppVersionSqlOperate, OperateResult,
};
use crate::ceobe_operation::app_version::models::model_app_version;

impl CeobeOperationAppVersionSqlOperate {
    pub async fn is_exist_app_version(
        version: &impl AsRef<str>, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
        model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version.as_ref()))
            .select_only()
            .column_as(model_app_version::Column::Id.count(), "count")
            .into_model::<AppVersionCounts>()
            .one(db)
            .await?
            .unwrap()
            .pipe(|AppVersionCounts { count }| count != 0)
            .tap(|result| {
                info!(
                    version = version.as_ref(),
                    result = if *result { "Exist" } else { "Not Exist" }
                )
            })
            .pipe(Ok)
    }
}
