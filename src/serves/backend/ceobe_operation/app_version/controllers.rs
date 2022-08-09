use axum_prehandle::{
    prefabs::json::JsonPayload, PreHandling, PreRespHandling,
};
use orm_migrate::sql_models::ceobe_operation::app_version::{
    checkers::app_version_data::{
        CeobeOperationAppVersionChecker, CeobeOperationAppVersionUncheck,
    },
    operate::CeobeOperationAppVersionSqlOperate,
};

use super::error::{AppRespResult, CeobeOperationAppVersionError};
use crate::{router::CeobeOpVersion, utils::data_checker::PreLiteChecker};

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
        CeobeOperationAppVersionSqlOperate::create_one_version(version)
            .await?;

        Ok(()).into()
    }
}