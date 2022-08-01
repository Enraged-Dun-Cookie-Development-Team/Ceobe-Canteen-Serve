use axum_prehandle::{
    prefabs::json::JsonPayload, PreHandling, PreRespHandling,
};
use mongo_migration::mongo_models::ceobe_operation::plugin_version::{
    check::plugin_version_checker::{
        PluginVersionChecker, PluginVersionUncheck,
    },
    operates::PluginDbOperation,
};

use super::error::{CeobeOperationPluginVersionError, PluginRespResult};
use crate::{
    router::CeobeOpVersion, utils::data_checker::PreLiteChecker,
};

type PluginVersionPreChecker = PreLiteChecker<
    JsonPayload<PluginVersionUncheck>,
    PluginVersionChecker,
    CeobeOperationPluginVersionError,
>;

impl CeobeOpVersion {
    pub async fn update_plugin(
        PreHandling(version): PreRespHandling<PluginVersionPreChecker>,
    ) -> PluginRespResult<()> {
        PluginDbOperation::update_new(version).await?;

        Ok(()).into()
    }
}
