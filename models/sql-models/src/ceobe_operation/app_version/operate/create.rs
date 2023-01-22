use sea_orm::{ActiveModelTrait, ConnectionTrait};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::{Pipe, Tap};
use tracing::{info, instrument};

use super::{
    CeobeOperationAppVersionSqlOperate, OperateError, OperateResult,
};
use crate::ceobe_operation::app_version::{
    checkers::app_version_data::CeobeOperationAppVersion,
    models::model_app_version,
    operate::create::model_app_version::ActiveModel,
};

impl CeobeOperationAppVersionSqlOperate {
    #[instrument(skip(db), ret)]
    pub async fn create_one_version<'db, D>(
        db: &'db D, version_info: CeobeOperationAppVersion,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        info!(
            newVersion.version = version_info.version,
            newVersion.force = version_info.force
        );

        let db = db.get_connect();
        // 判断版本是否已存在

        let false = Self::is_exist_app_version(&version_info.version, db).await? else {
            return Err(OperateError::AppVersionIdExist(version_info.version));
        };

        ActiveModel::create_app_version(version_info)
            .pipe(|active| active.insert(db))
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
