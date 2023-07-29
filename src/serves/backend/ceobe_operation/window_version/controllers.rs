use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::{models::window_version, ToCeobe, ToCeobeOperation},
    mysql::SqlDatabaseOperate,
};
use resp_result::resp_try;
use tracing::instrument;

use super::error::{WindowRespResult, CeobeOperationWindowVersionError};
use crate::router::CeobeOpVersion;

type CreateWindowVersionCheck =
    JsonCheckExtract<window_version::Checker, CeobeOperationWindowVersionError>;

impl CeobeOpVersion {
    // 新增一个桌面端版本
    #[instrument(ret, skip(db))]
    pub async fn create_window_version(
        db: SqlDatabaseOperate, CheckExtract(version): CreateWindowVersionCheck,
    ) -> WindowRespResult<()> {
        resp_try(async {
            db.ceobe()
                .operation()
                .window_version()
                .create_one(version)
                .await?;

            Ok(())
        })
        .await
    }
}
