use tencent_cloud_server::cdn::purge_urls_cache::PurgeCachePath;

pub struct TencentCDNPath;

impl TencentCDNPath {
    pub const LATEST_VERSION: PurgeCachePath =
        PurgeCachePath::new("/cdn/operate/version/fetch");
}