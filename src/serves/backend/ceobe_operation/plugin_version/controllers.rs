use checker::{CheckExtract, JsonCheckExtract};
use persistence::{
    ceobe_operate::{plugin_version::Checker, ToCeobe, ToCeobeOperation},
    mongodb::MongoDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use axum_resp_result::resp_try;
use tracing::instrument;

use super::error::{CeobeOperationPluginVersionError, PluginRespResult};
use crate::router::CeobeOpVersion;

type PluginVersionPreChecker =
    JsonCheckExtract<Checker, CeobeOperationPluginVersionError>;

impl CeobeOpVersion {
    #[instrument(ret, skip(db))]
    pub async fn update_plugin(
        db: MongoDatabaseOperate,
        CheckExtract(version): PluginVersionPreChecker,
    ) -> PluginRespResult<()> {
        resp_try(async {
            db.ceobe()
                .operation()
                .plugin_version()
                .update_new(version.clone())
                .await?;
            db.ceobe()
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
