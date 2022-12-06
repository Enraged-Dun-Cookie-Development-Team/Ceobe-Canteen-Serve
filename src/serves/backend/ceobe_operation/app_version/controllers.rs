use checker::{CheckExtract, JsonCheckExtract};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::ceobe_operation::app_version::{
        checkers::app_version_data::CeobeOperationAppVersionChecker,
        operate::CeobeOperationAppVersionSqlOperate,
    },
};
use resp_result::resp_try;
use tracing::instrument;

use super::error::{AppRespResult, CeobeOperationAppVersionError};
use crate::router::CeobeOpVersion;

type CreateAppVersionCheck = JsonCheckExtract<
    CeobeOperationAppVersionChecker,
    CeobeOperationAppVersionError,
>;

impl CeobeOpVersion {
    // 新增一个app版本
    #[instrument(ret, skip(db))]
    pub async fn create_app_version(
        db: SqlConnect, CheckExtract(version): CreateAppVersionCheck,
    ) -> AppRespResult<()> {
        resp_try(async {
            CeobeOperationAppVersionSqlOperate::create_one_version(
                &db, version,
            )
            .await?;

            Ok(())
        })
        .await
    }
}
