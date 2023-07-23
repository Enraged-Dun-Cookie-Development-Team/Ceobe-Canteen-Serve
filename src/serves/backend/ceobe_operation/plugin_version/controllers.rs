use persistence::ceobe_operate::{ToCeobe, ToCeobeOperation};
use checker::{CheckExtract, JsonCheckExtract};

use resp_result::resp_try;
use tracing::instrument;
use persistence::ceobe_operate::plugin_version::Checker;
use persistence::mongodb::MongoDatabaseOperate;

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
