use db_ops_prelude::{
    ext_traits::select_count::QueryCountByColumn,
    get_connect::GetDatabaseConnect,
    sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter},
    tap::{Pipe, Tap},
};
use tracing::info;

use super::{Column, Entity, OperateResult, DesktopVersionOperate};

impl<'c, C> DesktopVersionOperate<'c, C>
where
    C: 'c + GetDatabaseConnect,
{
    pub async fn is_exist_desktop_version(
        version: &impl AsRef<str>, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
        Entity::find()
            .filter(Column::Version.eq(version.as_ref()))
            .count_non_zero_by_column(Column::Id)
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
