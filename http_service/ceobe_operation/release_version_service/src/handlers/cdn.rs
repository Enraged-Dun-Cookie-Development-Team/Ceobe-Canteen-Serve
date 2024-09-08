use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use persistence::ceobe_operate::models::version::models::ReleaseVersion;
use serve_utils::{
    axum::extract::Query,
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    OptionValueField,
};

use super::{MapRejecter, Result};
use crate::view::QueryReleaseVersion;
impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn release_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryReleaseVersion {
            version: OptionValueField(version),
            platform,
        }): MapRejecter<Query<QueryReleaseVersion>>,
    ) -> Result<ReleaseVersion> {
        let release_info = logic.fetch(version, platform).await?;

        Ok(release_info)
    }
}
