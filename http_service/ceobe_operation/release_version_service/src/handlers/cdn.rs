use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::SerdeCheck;
use page_size::response::ListWithPageInfo;
use persistence::ceobe_operate::models::version::models::{
    ReleasePlatform, ReleaseVersion,
};
use serve_utils::{
    axum::extract::Query,
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    OptionField, ValueField,
};

use super::{MapRejecter, Result};
use crate::view::{QueryReleaseVersion, QueryVersionFilter};
impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn fetch_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryReleaseVersion {
            version: OptionField(version),
            platform,
        }): MapRejecter<Query<QueryReleaseVersion>>,
    ) -> Result<ReleaseVersion> {
        let release_info = logic.fetch(version, platform).await?;

        Ok(release_info)
    }


}
