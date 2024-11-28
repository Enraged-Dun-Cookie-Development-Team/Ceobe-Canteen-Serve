use semver::Version;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::ceobe::operation::version::models::{
    download_source::DownloadSourceItem, force::ForceCtrl,
    platform::ReleasePlatform,
};

#[derive(Debug, Serialize, Deserialize, Clone, TypedBuilder, PartialEq)]
#[builder(mutators(
    /// 一次添加一个下载源
    pub fn add_download_source(&mut self, source: DownloadSourceItem){
        self.download_source.push(source)
    }
    /// 一次添加多个下载源
    pub fn extend_download_source(&mut self, sources: impl IntoIterator<Item=DownloadSourceItem>){
        self.download_source.extend(sources)
    }
    )
)]
pub struct ReleaseVersion {
    /// 当前要发布的版本号
    pub version: Version,
    /// 发布的版本更新控制
    force: ForceCtrl,
    /// 发布的版本的说明
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 该版本发布的平台
    pub platform: ReleasePlatform,
    /// 该版本的可用下载源
    #[builder(via_mutators)]
    download_source: Vec<DownloadSourceItem>,
    #[builder(default = false)]
    #[serde(default)]
    deleted: bool,
}

#[cfg(test)]
mod test {
    use semver::Version;
    use serde_json::json;

    use crate::ceobe::operation::version::models::{
        platform::SupportPlatform, primary::Primary, DownloadSourceItem,
        ForceCtrl, ReleasePlatform, ReleaseVersion, ResourceUrl,
    };

    #[test]
    fn test_version_serde() {
        let ver = ReleaseVersion::builder()
            .version(Version::new(1, 13, 2))
            .force(
                ForceCtrl::builder()
                    .previous_force_version(Version::new(1, 0, 0))
                    .build(),
            )
            .description("Abc")
            .platform(ReleasePlatform::Desktop)
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("百度云盘")
                    .description("PanBaidu")
                    .primary_url(
                        ResourceUrl::builder()
                            .url(
                                "https://pan.baidu.com/s/114514"
                                    .parse()
                                    .unwrap(),
                            )
                            .name(Primary)
                            .manual()
                            .add_support_platform(SupportPlatform::Firefox)
                            .add_support_platform(SupportPlatform::Safari)
                            .build(),
                    )
                    .add_spare_url(
                        ResourceUrl::builder()
                            .url(
                                "https://pan.baidu.com/s/1919810"
                                    .parse()
                                    .unwrap(),
                            )
                            .name("百度云备用")
                            .build(),
                    )
                    .build(),
            )
            .build();

        let serde =
            serde_json::to_value(ver.clone()).expect("serde json error");
        assert_eq!(
            serde,
            json!({
                "version": "1.13.2",
                "force": {
                    "force_update": false,
                    "previous_force_version": "1.0.0"
                },
                "description": "Abc",
                "platform": "desktop",
                "download_source": [
                    {
                        "name": "百度云盘",
                        "description": "PanBaidu",
                        "primary_url": {
                            "url":"https://pan.baidu.com/s/114514",
                            "manual":true,
                            "support_platforms":["Firefox","Safari"]
                        },
                        "spare_urls": [
                            {
                                "name": "百度云备用",
                                "url": "https://pan.baidu.com/s/1919810",
                                "manual":false
                            }
                    ]
                    }
                ]

            })
        );

        let ver_de = serde_json::from_value(serde).expect("Deserailze_err");
        assert_eq!(ver, ver_de)
    }
}
