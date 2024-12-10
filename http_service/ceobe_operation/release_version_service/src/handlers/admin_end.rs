use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::SerdeCheck;
use page_size::response::ListWithPageInfo;
use persistence::{ceobe_operate::models::version::models::ReleaseVersion, ceobe_user::property::delete};
use serve_utils::{
    axum::{extract::Query, Json}, axum_resp_result::{resp_result, MapReject}, const_field::ConstBoolField, tracing::instrument, OptionField, ValueField
};

use crate::{
    handlers::{MapRejecter, Result},
    view::{QueryReleaseVersion, QueryVersionFilter, QueryVersionUpdate, UpdatePayload},
};

impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn mark_delete_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryReleaseVersion {
            version: ValueField(version),
            platform,
        }): MapRejecter<
            Query<QueryReleaseVersion<ValueField<semver::Version>>>,
        >,
    ) -> Result<()> {
        logic.mark_deleted(&version, &platform).await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all,fields(version = %(arg_1.0.version)))]
    pub async fn new_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(release): MapRejecter<Json<ReleaseVersion>>,
    ) -> Result<()> {
        logic.create_new(release).await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all,fields(
        version = %(arg_1.0.version),
    ))]
    pub async fn modify_description(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionUpdate {
            version:
                QueryReleaseVersion {
                    version: ValueField(version),
                    platform,
                },
            set:
                UpdatePayload {
                    description,
                    download_source,
                },
        }): MapRejecter<Json<QueryVersionUpdate>>,
    ) -> Result<()> {
        logic
            .update(version, platform, description, download_source)
            .await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionFilter {
            platform: OptionField(platform),
            deleted: ValueField(deleted),
            paginator: SerdeCheck(paginator),
        }): MapRejecter<Query<QueryVersionFilter<ValueField<bool>>>>,
    ) -> Result<ListWithPageInfo<ReleaseVersion>> {
        let ret = logic.all(paginator.into(), platform, deleted).await?;

        Ok(ret)
    }
    
}
