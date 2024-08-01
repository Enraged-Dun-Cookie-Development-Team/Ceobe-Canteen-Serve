use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use super::primary::SkipPrimarySerialize;
use crate::ceobe::operation::version::models::{
    platform::SupportPlatform, primary::Primary,
};
/// 可供使用的下载源
#[derive(Debug, Serialize, Clone, Deserialize, TypedBuilder, PartialEq)]
#[builder(mutators(
    /// 一次添加一个备用下载源
    pub fn add_spare_url(&mut self, spare: ResourceUrl){
        self.spare_urls.push(spare)
    }
    /// 一次添加多个备用下载源
    pub fn extend_spare_url(
        &mut self,
        spares: impl IntoIterator<Item=ResourceUrl>
    ){
        self.spare_urls.extend(spares)
    }
),doc)]
pub struct DownloadSourceItem {
    /// 下载源的名称，例如 “百度云盘”、“Github”等
    #[builder(setter(
        doc = "下载源的名称，例如 “百度云盘”、“Github”等",
        into
    ))]
    name: String,
    /// 下载源的描述，可选内容
    #[builder(default)]
    #[builder(setter(doc = "下载源的描述，可选内容", into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    description: Option<String>,
    /// 下载源的主要URL
    #[builder(setter(doc = "下载源的主要URL"))]
    primary_url: ResourceUrl<Primary>,
    /// 下载源的备用URL,可空
    #[builder(via_mutators)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    spare_urls: Vec<ResourceUrl>,
}

/// 下载源的备用URL
#[derive(Debug, Clone, Deserialize, Serialize, TypedBuilder, PartialEq)]
#[builder(doc, mutators(
    pub fn add_support_platform(&mut self, platform: SupportPlatform){
        self.support_platforms.push(platform)
    }
))]
pub struct ResourceUrl<Name = String> {
    /// 下载源备用URL的名称
    #[builder(setter(doc = "下载源URL的名称", into))]
    #[serde(
        skip_serializing_if = "SkipPrimarySerialize::should_skip",
        bound = "for<'d>Name: SkipPrimarySerialize + \
                 Serialize+Deserialize<'d>",
        default
    )]
    name: Name,
    /// 下载源备用URL的URL
    #[builder(setter(doc = "下载源URL的URL"))]
    url: Url,
    #[builder(setter(strip_bool))]
    manual: bool,
    #[builder(via_mutators)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    support_platforms: Vec<SupportPlatform>,
}

#[cfg(test)]
mod test {
    use crate::ceobe::operation::version::models::download_source::{
        DownloadSourceItem, Primary, ResourceUrl,
    };
    #[test]
    fn test_construct_spare() {
        let item = DownloadSourceItem::builder()
            .extend_spare_url([ResourceUrl::builder()
                .url("http://example.com".parse().unwrap())
                .name("example backup".to_string())
                .build()])
            .name("ABC".to_string())
            .primary_url(
                ResourceUrl::builder()
                    .url("http://primary.com".parse().unwrap())
                    .name(Primary)
                    .build(),
            )
            .build();
        println!("{item:?}")
    }
    #[test]
    fn test_primary_serde() {
        let a = serde_json::to_string(&Primary).unwrap();
        assert_eq!(a, "\"primary\"")
    }
}
