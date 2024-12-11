use db_ops_prelude::{
    database_operates::operate_trait::OperateTrait,
    mongodb::bson::oid::ObjectId,
};
use page_next_id::response::{GenerateListWithNextId, ListWithNextId};
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
use tokio::task;

use super::{LogicResult, ReleaseVersionLogic, TencentCDNPath};

impl ReleaseVersionLogic {
    async fn purge_version_cache(
        &self, version: &Option<Version>, platform: &ReleasePlatform,
    ) -> LogicResult<()> {
        self.tencent_cloud
            .purge_urls_cache(&[
                // 最新版本
                TencentCDNPath::LATEST_VERSION(&None, platform)?,
                // 当前版本
                TencentCDNPath::LATEST_VERSION(version, platform)?,
                // 分页第一页
                TencentCDNPath::VERSION_LIST(&None, platform)?,
            ])
            .await?;

        Ok(())
    }

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

        self.purge_version_cache(&Some(version.clone()), platform)
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
            .one(release.clone())
            .await?;
        self.purge_version_cache(&Some(release.version), &release.platform)
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
        self.purge_version_cache(&Some(version), &platform).await?;
        Ok(())
    }

    pub async fn all_by_page_id(
        &self, first_id: Option<ObjectId>, platform: ReleasePlatform,
        deleted: bool,
    ) -> LogicResult<ListWithNextId<ReleaseVersion, String>> {
        let list = task::spawn({
            let mongodb = self.mongodb.clone();
            async move {
                mongodb
                    .ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .all_by_first_id(Some(platform), first_id, deleted, 10)
                    .await
            }
        });
        let next_id = task::spawn({
            let mongodb = self.mongodb.clone();
            async move {
                mongodb
                    .ceobe()
                    .operation()
                    .release_version()
                    .retrieve()
                    .get_next_id(Some(platform), first_id, deleted, 10)
                    .await
            }
        });

        let list = list.await??;
        let next_id = next_id.await??;

        Ok(list.with_page_next_id_info(next_id.map(|id| id.to_string())))
    }
}
