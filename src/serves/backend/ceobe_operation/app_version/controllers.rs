use axum_prehandle::{PreHandling, PreRespHandling, prefabs::json::JsonPayload};

use orm_migrate::sql_models::ceobe_operation::app_version::{checkers::app_version_data::{CeobeOperationAppVersionChecker, CeobeOperationAppVersionUncheck}, operate::CeobeOperationAppVersionSqlOperate};

use crate::{router::CeobeOpVersion, utils::data_checker::PreLiteChecker};

use super::error::{AppRespResult, CeobeOperationAppVersionError};

type CreateAppVersionCheck = PreLiteChecker<
    JsonPayload<CeobeOperationAppVersionUncheck>,
    CeobeOperationAppVersionChecker,
    CeobeOperationAppVersionError,
>;

impl CeobeOpVersion {
    // 新增一个app版本
    pub async fn create_app_version(
        PreHandling(version): PreRespHandling<CreateAppVersionCheck>,
    ) -> AppRespResult<()> {
        CeobeOperationAppVersionSqlOperate::create_one_version(version).await?;

        Ok(()).into()
    }
}