use std::ops::Deref;

use db_ops_prelude::{
    bool_or::FalseOrError,
    get_connect::GetDatabaseConnect,
    sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel},
    tap::{Pipe, Tap},
};
use tracing::{info, instrument};

use super::{WindowVersionOperate, Checked, OperateError, OperateResult};

impl<'c, C> WindowVersionOperate<'c, C>
where
    C: GetDatabaseConnect + 'c,
    C::Connect: ConnectionTrait,
{
    #[instrument(skip(self), ret)]
    pub async fn create_one(
        &self, version_info: Checked,
    ) -> OperateResult<()> {
        info!(
            newVersion.version = version_info.version,
            newVersion.force = version_info.force
        );

        let db = self.deref();

        // 判断版本是否已存在
        Self::is_exist_window_version(&version_info.version, db)
            .await?
            .false_or_with(|| {
                OperateError::WindowVersionIdExist(version_info.version.clone())
            })?;

        version_info
            .into_active_model()
            .insert(db)
            .await?
            .tap(|result| {
                info!(
                    newVersion.store = true,
                    newVersion.version = result.version,
                    newVersion.force = result.force
                )
            })
            .pipe(|_| Ok(()))
    }
}
