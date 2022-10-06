use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};
use sql_connection::database_traits::get_connect::GetDatabaseConnect;
use tap::Pipe;

use super::{
    CeobeOperationAppVersionSqlOperate, OperateError, OperateResult,
};
use crate::ceobe_operation::app_version::{
    checkers::app_version_data::CeobeOperationAppVersion,
    models::model_app_version,
    operate::create::model_app_version::ActiveModel,
};

impl CeobeOperationAppVersionSqlOperate {
    pub async fn create_one_version<'db, D>(
        db: &'db D, version_info: CeobeOperationAppVersion,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'static,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        // 判断版本是否已存在
        if Self::is_exist_app_version(&version_info.version, db).await? {
            Err(OperateError::AppVersionIdExist(version_info.version))
        }
        else {
            ActiveModel::create_app_version(version_info)
                .pipe(|active| active.insert(db))
                .await?
                .pipe(|_| Ok(()))
        }
    }
}
