use axum_resp_result::resp_try;
use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::{models::desktop_version, ToCeobe, ToCeobeOperation},
    mongodb::MongoDatabaseOperate,
    mysql::SqlDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use tracing::instrument;

use super::error::{CeobeOperationDesktopVersionError, DesktopRespResult};
use crate::router::CeobeOpVersion;

type CreateDesktopVersionCheck = JsonCheckExtract<
    desktop_version::Checker,
    CeobeOperationDesktopVersionError,
>;

impl CeobeOpVersion {
    // 新增一个桌面端版本
    #[instrument(ret, skip(db, mongo))]
    pub async fn create_desktop_version(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate,
        CheckExtract(version): CreateDesktopVersionCheck,
    ) -> DesktopRespResult<()> {
        resp_try(async {
            db.ceobe()
                .operation()
                .desktop_version()
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
