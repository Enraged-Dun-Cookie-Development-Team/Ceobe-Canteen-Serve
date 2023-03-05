use abstract_database::ceobe::ToCeobe;
use ceobe_operate::ToCeobeOperation;
use checker::{CheckExtract, JsonCheckExtract};
use mongo_migration::{
    mongo_connection::MongoDatabaseOperate,
    mongo_models::ceobe::operation::plugin_version::Checker,
};
use resp_result::resp_try;
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
                .update_new(version)
                .await?;

            Ok(())
        })
        .await
    }
}
