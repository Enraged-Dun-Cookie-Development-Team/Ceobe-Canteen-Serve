use checker::{CheckExtract, JsonCheckExtract};
use mongo_migration::{
    mongo_connection::MongoConnect,
    mongo_models::ceobe_operation::plugin_version::{
        check::plugin_version_checker::PluginVersionChecker,
        operates::PluginDbOperation,
    },
};
use resp_result::resp_try;
use tracing::instrument;

use super::error::{CeobeOperationPluginVersionError, PluginRespResult};
use crate::router::CeobeOpVersion;

type PluginVersionPreChecker =
    JsonCheckExtract<PluginVersionChecker, CeobeOperationPluginVersionError>;

impl CeobeOpVersion {
    #[instrument(ret, skip(db))]
    pub async fn update_plugin(
        db: MongoConnect, CheckExtract(version): PluginVersionPreChecker,
    ) -> PluginRespResult<()> {
        resp_try(async {
            PluginDbOperation::update_new(&db, version).await?;

            Ok(())
        })
        .await
    }
}
