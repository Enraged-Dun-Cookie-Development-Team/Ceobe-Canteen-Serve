use db_ops_prelude::mongodb::bson::oid::ObjectId;
use persistence::ceobe_operate::models::version::models::ReleasePlatform;
use semver::Version;
use serde::Serialize;
use tencent_cloud_server::cdn::purge_urls_cache::PurgeCachePath;

pub struct TencentCDNPath;

impl TencentCDNPath {
    /// 版本信息
    #[allow(non_snake_case)]
    pub fn LATEST_VERSION(
        version: &Option<Version>, platform: &ReleasePlatform,
    ) -> Result<PurgeCachePath, serde_qs::Error> {
        #[derive(Serialize)]
        struct VersionInfoQuery<'a, 'b> {
            #[serde(skip_serializing_if = "Option::is_none")]
            version: &'a Option<Version>,
            platform: &'b ReleasePlatform,
        }

        PurgeCachePath::new_with_query(
            "/cdn/operate/version/fetch",
            &VersionInfoQuery { version, platform },
        )
    }

    /// 版本列表
    #[allow(non_snake_case)]
    pub fn VERSION_LIST(
        first_id: &Option<ObjectId>, platform: &ReleasePlatform,
    ) -> Result<PurgeCachePath, serde_qs::Error> {
        #[derive(Serialize)]
        struct VersionListQuery<'a, 'b> {
            #[serde(skip_serializing_if = "Option::is_none")]
            first_id: &'a Option<ObjectId>,
            platform: &'b ReleasePlatform,
        }

        PurgeCachePath::new_with_query(
            "/cdn/operate/version/all",
            &VersionListQuery { first_id, platform },
        )
    }
}
