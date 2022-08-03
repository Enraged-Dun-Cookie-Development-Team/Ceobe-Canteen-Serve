
use sql_connection::get_sql_database;
use crate::ceobe_operation::app_version::models::model_app_version;
use crate::ceobe_operation::app_version::{checkers::app_version_data::CeobeOperationAppVersion};

use sea_orm::ActiveModelTrait;
use super::{CeobeOperationAppVersionSqlOperate, OperateResult};

impl CeobeOperationAppVersionSqlOperate {
    pub async fn create_one_version(version: CeobeOperationAppVersion) -> OperateResult<()> {
        let db = get_sql_database();
        let mut active = <model_app_version::ActiveModel as std::default::Default>::default();
        active.create_app_version(version);
        active.insert(db).await?;
        Ok(())
    }
}
