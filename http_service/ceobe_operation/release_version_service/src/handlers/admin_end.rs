use checker::{prefabs::option_checker::OptionChecker, QueryCheckExtract};
use page_size::request::PageSizeChecker;
use persistence::{
    ceobe_operate::{
        models::version::models::ReleaseVersion, ToCeobe, ToCeobeOperation,
    },
    mongodb::MongoDatabaseOperate,
    operate::operate_trait::OperateTrait,
};
use serve_utils::{
    axum::{extract::Query, Json},
    axum_resp_result::{MapReject, resp_result},
    tracing::instrument,
    ValueField,
};
use tencent_cloud_server::cloud_manager::TencentCloudManager;

use crate::{
    error::Error,
    handlers::{MapRejecter, Result},
    view::{QueryReleaseVersion, QueryVersionFilter, TencentCDNPath},
};

impl crate::ReleaseVersionController {
    #[resp_result]
    #[instrument(skip_all,fields(version = %arg_2.0))]
    pub async fn yank_version(
        db: MongoDatabaseOperate, tencent_cloud: TencentCloudManager,
        MapReject(QueryReleaseVersion {
            version: ValueField(version),
            platform,
        }): MapRejecter<
            Query<QueryReleaseVersion<ValueField<semver::Version>>>,
        >,
    ) -> Result<()> {
        db.ceobe()
            .operation()
            .release_version()
            .delete()
            .yank(&platform, &version)
            .await?;
        tencent_cloud
            .purge_urls_cache(&Some(TencentCDNPath::LATEST_VERSION))
            .await?;
        Ok(())
    }

    #[resp_result]
    #[instrument(skip_all)]
    pub async fn all_version(
        db: MongoDatabaseOperate,
        MapReject(paginator): MapRejecter<
            QueryCheckExtract<OptionChecker<PageSizeChecker>, Error>,
        >,
        MapReject(filter): MapRejecter<Json<Option<QueryVersionFilter>>>,
    ) -> Result<Vec<ReleaseVersion>> {
        let filter = filter.unwrap_or_default();
        let ret = db
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .all(filter.platform, paginator, filter.yanked)
            .await?;

        Ok(ret)
    }

    #[resp_result]
    #[instrument(skip_all)]
    pub async fn released_version_num(
        db: MongoDatabaseOperate,
        MapReject(filter): MapRejecter<Json<Option<QueryVersionFilter>>>,
    ) -> Result<usize> {
        let filter = filter.unwrap_or_default();
        let ret = db
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .total_num(filter.platform, filter.yanked)
            .await?;
        Ok(ret)
    }

    #[resp_result]
    #[instrument(skip_all,fields(version = %(arg_2.0.version)))]
    pub async fn new_version(
        db: MongoDatabaseOperate, tencent_cloud: TencentCloudManager,
        MapReject(release): MapRejecter<Json<ReleaseVersion>>,
    ) -> Result<()> {
        db.ceobe()
            .operation()
            .release_version()
            .create()
            .one(release)
            .await?;
        tencent_cloud
            .purge_urls_cache(&Some(TencentCDNPath::LATEST_VERSION))
            .await?;
        Ok(())
    }
}
