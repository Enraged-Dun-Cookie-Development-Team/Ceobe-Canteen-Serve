use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::{
    prefabs::option_checker::OptionChecker, CheckExtract, QueryCheckExtract,
};
use page_size::{request::PageSizeChecker, response::ListWithPageInfo};
use persistence::ceobe_operate::models::version::models::ReleaseVersion;
use serve_utils::{
    axum::{extract::Query, Json},
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    ValueField,
};
use tracing::log::__private_api::log;

use crate::{
    error::Error,
    handlers::{MapRejecter, Result},
    view::{
        QueryReleaseVersion, QueryVersionFilter, QueryVersionUpdate,
        UpdateDescription, UpdateDownloadResource,
    },
};

impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_1.0))]
    pub async fn yank_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryReleaseVersion {
            version: ValueField(version),
            platform,
        }): MapRejecter<
            Query<QueryReleaseVersion<ValueField<semver::Version>>>,
        >,
    ) -> Result<()> {
        logic.yank(&version, &platform).await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        CheckExtract(paginator): QueryCheckExtract<
            OptionChecker<PageSizeChecker>,
            Error,
        >,
        MapReject(filter): MapRejecter<Json<Option<QueryVersionFilter>>>,
    ) -> Result<ListWithPageInfo<ReleaseVersion>> {
        let QueryVersionFilter { platform, yanked } =
            filter.unwrap_or_default();
        let ret = logic.all(paginator, platform, yanked).await?;

        Ok(ret)
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
            set: UpdateDescription { description },
        }): MapRejecter<Json<QueryVersionUpdate<UpdateDescription>>>,
    ) -> Result<()> {
        logic
            .update_description(version, platform, description)
            .await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all,fields(
        version = %(arg_1.0.version),
    ))]
    pub async fn modify_resource(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(QueryVersionUpdate {
            version:
                QueryReleaseVersion {
                    version: ValueField(version),
                    platform,
                },
            set: UpdateDownloadResource { download_source },
        }): MapRejecter<
            Json<QueryVersionUpdate<UpdateDownloadResource>>,
        >,
    ) -> Result<()> {
        logic
            .update_resource(version, platform, download_source)
            .await?;
        Ok(())
    }
}
