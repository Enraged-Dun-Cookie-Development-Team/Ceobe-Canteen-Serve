use axum_prehandle::{PreHandling, PreRespHandling};

use crate::{
    models::sql::app_version::operate::CeobeOperationAppVersionSqlOperate,
    router::CeobeOperationVersionFrontend,
};

use super::{models::OptionAppVersionCheckerPretreat, error::VersionRespResult, view::AppVersionView};

impl CeobeOperationVersionFrontend {
    pub async fn app_version(
        PreHandling(version): PreRespHandling<
            OptionAppVersionCheckerPretreat,
        >,
    ) -> VersionRespResult<AppVersionView> {
        let version = version.version;
        match version {
            Some(version) => {
                Ok(CeobeOperationAppVersionSqlOperate::get_app_version_info_by_version(version).await?.into()).into()
            }
            None => {
                Ok(CeobeOperationAppVersionSqlOperate::get_newest_app_version_info().await?.into()).into()
            }
        }
    }
}
