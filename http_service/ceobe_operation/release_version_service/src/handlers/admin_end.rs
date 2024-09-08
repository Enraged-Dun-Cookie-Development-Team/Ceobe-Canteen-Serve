use ceobe_operation_logic::{
    release_version::ReleaseVersionLogic, CeobeOperationLogic,
};
use checker::{
    prefabs::option_checker::OptionChecker, CheckExtract, QueryCheckExtract,
};
use page_size::{request::PageSizeChecker, response::ListWithPageInfo};
use persistence::{
    ceobe_operate::{
        models::version::models::ReleaseVersion, ToCeobe, ToCeobeOperation,
    },
    mongodb::MongoDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use serve_utils::{
    axum::{extract::Query, Json},
    axum_resp_result::{resp_result, MapReject},
    tracing::instrument,
    ValueField,
};
use tencent_cloud_server::cloud_manager::TencentCloudManager;

use crate::{
    error::Error,
    handlers::{MapRejecter, Result},
    view::{QueryReleaseVersion, QueryVersionFilter},
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
    #[instrument(skip_all)]
    pub async fn released_version_num(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(filter): MapRejecter<Json<Option<QueryVersionFilter>>>,
    ) -> Result<usize> {
        let QueryVersionFilter { platform, yanked } = filter.unwrap_or_default();
        let ret =logic.count(platform, yanked).await?; 
        Ok(ret)
    }

    #[resp_result]
    #[instrument(skip_all,fields(version = %(arg_1.0.version)))]
    pub async fn new_version(
        logic: CeobeOperationLogic<ReleaseVersionLogic>,
        MapReject(release): MapRejecter<Json<ReleaseVersion>>,
    ) -> Result<()> {
        logic.new(release).await?;
        Ok(())
    }
}
