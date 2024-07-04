use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

/// 可供使用的下载源
#[derive(Debug, Serialize, Clone, Deserialize, TypedBuilder)]
#[builder(mutators(
    /// 一次添加一个备用下载源
    pub fn add_spare_url(&mut self, spare: SpareUrl){
        self.spare_urls.push(spare)
    }
    /// 一次添加多个备用下载源
    pub fn extend_spare_url(&mut self, spares: impl IntoIterator<Item=SpareUrl>){
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
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 下载源的主要URL
    #[builder(setter(doc = "下载源的主要URL"))]
    primary_url: Url,
    /// 下载源的备用URL,可空
    #[builder(via_mutators)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    spare_urls: Vec<SpareUrl>,
}

/// 下载源的备用URL
#[derive(Debug, Clone, Deserialize, Serialize, TypedBuilder)]
#[builder(doc)]
pub struct SpareUrl {
    /// 下载源备用URL的名称
    #[builder(setter(doc = "下载源备用URL的名称", into))]
    name: String,
    /// 下载源备用URL的描述，可选
    #[builder(default)]
    #[builder(setter(
        doc = "下载源备用URL的描述，可选",
        into,
        strip_option
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 下载源备用URL的URL
    #[builder(setter(doc = "下载源备用URL的URL"))]
    url: Url,
}

#[cfg(test)]
mod test {
    use crate::ceobe::operation::version::models::download_source::{
        DownloadSourceItem, SpareUrl,
    };
    #[test]
    fn test_construct_spare() {
        let item = DownloadSourceItem::builder()
            .extend_spare_url([SpareUrl::builder()
                .url("http://example.com".parse().unwrap())
                .name("example backup".to_string())
                .build()])
            .name("ABC".to_string())
            .primary_url("http://primary.com".parse().unwrap())
            .build();
        println!("{item:?}")
    }
}
