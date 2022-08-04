use sea_orm::ActiveModelTrait;
use sql_connection::get_sql_database;

use super::{CeobeOperationAppVersionSqlOperate, OperateResult, OperateError};
use crate::ceobe_operation::app_version::{
    checkers::app_version_data::CeobeOperationAppVersion,
    models::model_app_version,
    operate::create::model_app_version::ActiveModel,
};

impl CeobeOperationAppVersionSqlOperate {
    pub async fn create_one_version(
        version_info: CeobeOperationAppVersion,
    ) -> OperateResult<()> {
        let db = get_sql_database();
        // 判断版本是否已存在
        if Self::is_exist_app_version(version_info.version.clone(), db).await? {
            return Err(OperateError::AppVersionIdExist(version_info.version));
        }
        let active = ActiveModel::create_app_version(version_info);
        active.insert(db).await?;
        Ok(())
    }
}
