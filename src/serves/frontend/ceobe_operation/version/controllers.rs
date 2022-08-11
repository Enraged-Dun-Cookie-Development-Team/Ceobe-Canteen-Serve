use axum_prehandle::{PreHandling, PreRespHandling};
use mongo_migration::mongo_models::ceobe_operation::plugin_version::operates::PluginDbOperation;

use super::{
    error::VersionRespResult,
    models::{AppVersion, OptionAppVersionCheckerPretreat, OptionPluginVersionCheckerPretreat},
    view::{AppVersionView, PluginVersionView},
};
use crate::{
    models::sql::app_version::operate::CeobeOperationAppVersionSqlOperate,
    router::CeobeOperationVersionFrontend,
};

impl CeobeOperationVersionFrontend {
    // 获取app对应版本信息
    pub async fn app_version(
        PreHandling(AppVersion { version }): PreRespHandling<
            OptionAppVersionCheckerPretreat,
        >,
    ) -> VersionRespResult<AppVersionView> {
        match version {
            Some(version) => {
                Ok(CeobeOperationAppVersionSqlOperate::get_app_version_info_by_version(version).await?.into()).into()
            }
            None => {
                Ok(CeobeOperationAppVersionSqlOperate::get_newest_app_version_info().await?.into()).into()
            }
        }
    }

    // 获取插件端对应版本信息
    pub async fn plugin_version(
        PreHandling(version): PreRespHandling<
            OptionPluginVersionCheckerPretreat,
        >,
    ) -> VersionRespResult<PluginVersionView> {
        let version = version.version;
        match version {
            Some(version) => {
                Ok(PluginDbOperation::get_plugin_version_info_by_version(version).await?.into()).into()
            }
            None => {
                Ok(PluginDbOperation::get_newest_plugin_version_info().await?.into()).into()
            }
        }
    }
}
