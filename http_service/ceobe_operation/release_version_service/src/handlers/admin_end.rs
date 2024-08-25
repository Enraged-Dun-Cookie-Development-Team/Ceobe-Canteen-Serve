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
        MapReject(filter): MapRejecter<Query<Option<QueryVersionFilter>>>,
    ) -> Result<Vec<ReleaseVersion>> {
        let filter = filter.unwrap_or_default();
        let ret = db
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .all(filter.platform, filter.yanked)
            .await?;
        Ok(ret)
    }

    #[resp_result]
    #[instrument(skip_all,fields(version = %(arg_2.0.version)))]
    pub async fn new_version(
        db: MongoDatabaseOperate,
        tencent_cloud: TencentCloudManager,
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
