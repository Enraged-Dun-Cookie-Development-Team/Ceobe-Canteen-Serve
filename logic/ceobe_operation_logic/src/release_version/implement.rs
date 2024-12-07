use db_ops_prelude::database_operates::operate_trait::OperateTrait;
use page_size::{
    request::Paginator,
    response::{GenerateListWithPageInfo, ListWithPageInfo},
};
use persistence::ceobe_operate::{
    models::version::models::{
        DownloadSourceItem, ReleasePlatform, ReleaseVersion,
    },
    ToCeobe, ToCeobeOperation,
};
use semver::Version;

use super::{LogicResult, ReleaseVersionLogic, TencentCDNPath};

impl ReleaseVersionLogic {
    pub async fn mark_deleted(
        &self, version: &Version, platform: &ReleasePlatform,
    ) -> LogicResult<()> {
        self.mongodb
            .ceobe()
            .operation()
            .release_version()
            .update()
            .mark_deleted(platform, version)
            .await?;

        self.tencent_cloud
            .purge_urls_cache(&[
                TencentCDNPath::LATEST_VERSION,
                TencentCDNPath::VERSION_LIST,
            ])
            .await?;

        Ok(())
    }

    pub async fn all(
        &self, paginator: Option<Paginator>,
        platform: Option<ReleasePlatform>, deleted: bool,
    ) -> LogicResult<ListWithPageInfo<ReleaseVersion>> {
        let msg = self
            .mongodb
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .all(platform, paginator, deleted)
            .await?;

        match paginator {
            Some(paginator) => {
                let total = self.count(platform, deleted).await?;
                Ok(msg.with_page_info(paginator, total as _))
            }
            None => Ok(msg.with_plain()),
        }
    }

    async fn count(
        &self, platform: Option<ReleasePlatform>, deleted: bool,
    ) -> LogicResult<usize> {
        let count = self
            .mongodb
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .total_num(platform, deleted)
            .await?;
        Ok(count)
    }

    pub async fn create_new(
        &self, release: ReleaseVersion,
    ) -> LogicResult<()> {
        self.mongodb
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(release)
            .await?;
        self.tencent_cloud
            .purge_urls_cache(&[
                TencentCDNPath::LATEST_VERSION,
                TencentCDNPath::VERSION_LIST,
            ])
            .await?;
        Ok(())
    }

    pub async fn fetch(
        &self, version: Option<Version>, platform: ReleasePlatform,
    ) -> LogicResult<ReleaseVersion> {
        let release_info = match version {
            None => {
                self.mongodb
                    .ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .latest_by_platform(platform)
                    .await?
            }
            Some(ver) => {
                self.mongodb
                    .ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .by_version_platform(&ver, platform)
                    .await?
            }
        };
        Ok(release_info)
    }

    pub async fn update(
        &self, version: Version, platform: ReleasePlatform,
        description: Option<String>, resources: Vec<DownloadSourceItem>,
    ) -> LogicResult<()> {
        self.mongodb
            .ceobe()
            .operation()
            .release_version()
            .update()
            .description_and_resource(
                version.clone(),
                platform,
                description,
                resources,
            )
            .await?;
        self.tencent_cloud
            .purge_urls_cache(&[
                TencentCDNPath::LATEST_VERSION,
                TencentCDNPath::VERSION_LIST,
            ])
            .await?;
        Ok(())
    }
}
