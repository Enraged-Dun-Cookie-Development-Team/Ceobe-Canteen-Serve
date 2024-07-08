use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::{models::app_version, ToCeobe, ToCeobeOperation},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use axum_resp_result::resp_try;
use tracing::instrument;

use super::error::{AppRespResult, CeobeOperationAppVersionError};
use crate::router::CeobeOpVersion;

type CreateAppVersionCheck =
    JsonCheckExtract<app_version::Checker, CeobeOperationAppVersionError>;

impl CeobeOpVersion {
    // 新增一个app版本
    #[instrument(ret, skip(db, mongo))]
    pub async fn create_app_version(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        CheckExtract(version): CreateAppVersionCheck,
    ) -> AppRespResult<()> {
        resp_try(async {
            db.ceobe()
                .operation()
                .app_version()
                .create_one(version.clone())
                .await?;
            mongo
                .ceobe()
                .operation()
                .release_version()
                .create()
                .one(version)
                .await?;
            Ok(())
        })
        .await
    }
}
