use checker::{CheckExtract, JsonCheckExtract};
use database_traits::database_operates::sub_operate::SuperOperate;
use orm_migrate::{
    sql_connection::SqlDatabaseOperate,
    sql_models::ceobe_operation::{
        app_version::{
            checkers::app_version_data::CeobeOperationAppVersionChecker,
            operate::AppVersionOperate,
        },
        SqlCeobeOperation,
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
        mut db: SqlDatabaseOperate,
        CheckExtract(version): CreateAppVersionCheck,
    ) -> AppRespResult<()> {
        resp_try(async {
            db.child::<SqlCeobeOperation<_>>()
                .child::<AppVersionOperate<_>>()
                .create_one(version)
                .await?;

            Ok(())
        })
        .await
    }
}
