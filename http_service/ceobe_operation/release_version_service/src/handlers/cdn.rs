use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use page_next_id::response::ListWithNextId;
use persistence::{
    ceobe_operate::models::version::models::ReleaseVersion,
    mongodb::mongodb::bson::oid::ObjectId,
};
use serve_utils::{
    axum::extract::Query,
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    FetchOptionViewValue, FetchViewValue, OptionField,
};

use super::{MapRejecter, Result};
use crate::view::{QueryReleaseVersion, QueryVersionNextIdFilter};
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

    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version_by_next_id(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionNextIdFilter {
            platform,
            deleted,
            first_id,
        }): MapRejecter<Query<QueryVersionNextIdFilter>>,
    ) -> Result<ListWithNextId<ReleaseVersion, ObjectId>> {
        let ret = logic
            .all_by_page_id(
                first_id,
                platform.fetch_option(),
                deleted.fetch(),
            )
            .await?;

        Ok(ret)
    }
}
