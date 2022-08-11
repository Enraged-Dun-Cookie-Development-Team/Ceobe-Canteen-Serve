use std::time::Duration;

use axum_prehandle::{PreHandling, PreRespHandling};
use resp_result::RespResult;

use super::{
    error::FlagVersionRespResult,
    models::{
        AppVersion, OptionAppVersionCheckerPretreat,
        OptionPluginVersionCheckerPretreat,
    },
    view::{AppVersionView, PluginVersionView},
};
use crate::{
    models::{
        mongo::plugin_version::operates::PluginDbOperation,
        sql::app_version::operate::CeobeOperationAppVersionSqlOperate,
    },
    router::CeobeOperationVersionFrontend,
};

impl CeobeOperationVersionFrontend {
    // 获取app对应版本信息
    pub async fn app_version(
        PreHandling(AppVersion { version }): PreRespHandling<
            OptionAppVersionCheckerPretreat,
        >,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<AppVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        match version {
            Some(version) => {
                let (data, extra) = modify.check_modify(
                    CeobeOperationAppVersionSqlOperate::get_app_version_info_by_version(version).await?
                )?;
                RespResult::ok(data.map(Into::into)).with_flags(extra)
            }
            None => {
                let (data, extra) = modify.check_modify(
                    CeobeOperationAppVersionSqlOperate::get_newest_app_version_info().await?
                )?;
                RespResult::ok(data.map(Into::into)).with_flags(extra)
            }
        }
    }

    // 获取插件端对应版本信息
    pub async fn plugin_version(
        PreHandling(version): PreRespHandling<
            OptionPluginVersionCheckerPretreat,
        >,
        mut modify: modify_cache::CheckModify,
    ) -> FlagVersionRespResult<PluginVersionView> {
        let ctrl = modify.cache_headers.get_control();
        ctrl.set_max_age(Duration::from_secs(60 * 60));

        let version = version.version;
        match version {
            Some(version) => {
                let (data, extra) = modify.check_modify(
                    PluginDbOperation::get_plugin_version_info_by_version(
                        version,
                    )
                    .await?,
                )?;
                RespResult::ok(data.map(Into::into)).with_flags(extra)
            }
            None => {
                let (data, extra) = modify.check_modify(
                    PluginDbOperation::get_newest_plugin_version_info()
                        .await?,
                )?;
                RespResult::ok(data.map(Into::into)).with_flags(extra)
            }
        }
    }
}
