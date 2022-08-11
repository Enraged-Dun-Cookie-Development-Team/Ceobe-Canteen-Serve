use axum_prehandle::{PreHandling, PreRespHandling};

use super::{
    error::VersionRespResult,
    models::{AppVersion, OptionAppVersionCheckerPretreat},
    view::AppVersionView,
};
use crate::{
    models::sql::app_version::operate::CeobeOperationAppVersionSqlOperate,
    router::CeobeOperationVersionFrontend,
};

impl CeobeOperationVersionFrontend {
    pub async fn app_version(
        PreHandling(AppVersion { version }): PreRespHandling<
            OptionAppVersionCheckerPretreat,
        >,
    ) -> VersionRespResult<AppVersionView> {
        Ok(match version {
                    Some(version) => {
                        CeobeOperationAppVersionSqlOperate::get_app_version_info_by_version(version).await?.into()
                    }
                    None => {
                        CeobeOperationAppVersionSqlOperate::get_newest_app_version_info().await?.into()
                    }
                }).into()
    }

    pub async fn plugin_version() {}
}
