use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use sql_connection::{
    database_traits::get_connect::GetDatabaseConnect,
    ext_traits::select_count::QueryCountByColumn,
};
use tap::{Pipe, Tap};
use tracing::info;

use super::{AppVersionOperate, OperateResult};
use crate::ceobe_operation::app_version::models::model_app_version;

impl<'c, C> AppVersionOperate<'c, C>
where
    C: 'c + GetDatabaseConnect,
{
    pub async fn is_exist_app_version(
        version: &impl AsRef<str>, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
        model_app_version::Entity::find()
            .filter(model_app_version::Column::Version.eq(version.as_ref()))
            .count_non_zero_by_column(model_app_version::Column::Id)
            .one(db)
            .await?
            .unwrap()
            .take()
            .tap(|result| {
                info!(
                    version = version.as_ref(),
                    result = if *result { "Exist" } else { "Not Exist" }
                )
            })
            .pipe(Ok)
    }
}
