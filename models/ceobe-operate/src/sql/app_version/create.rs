use std::ops::Deref;

use db_ops_prelude::{
    get_connect::GetDatabaseConnect,
    sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel},
    tap::{Pipe, Tap},
};
use tracing::{info, instrument};

use super::{AppVersionOperate, Checked, OperateError, OperateResult};
impl<'c, C> AppVersionOperate<'c, C>
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

        let false = Self::is_exist_app_version(&version_info.version,db).await? else {
            return Err(OperateError::AppVersionIdExist(version_info.version));
        };

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
