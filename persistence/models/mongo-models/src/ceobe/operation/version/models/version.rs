use semver::{Op, Version};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::ceobe::operation::version::models::download_sourece::DownloadSourceItem;
use crate::ceobe::operation::version::models::force::ForceCtrl;
use crate::ceobe::operation::version::models::platform::Platform;

#[derive(Debug,Serialize,Deserialize,Clone,TypedBuilder)]
#[builder(mutators(
    /// 一次添加一个下载源
    fn add_spare_url(&mut self, source: DownloadSourceItem){
        self.download_source.push(source)
    }
    /// 一次添加多个下载源
    fn extend_spare_url(&mut self, sources: impl IntoIterator<Item=DownloadSourceItem>){
        self.download_source.extend(sources)
    }
    )
)]
pub struct ReleaseVersion{
    /// 当前要发布的版本号
    version:Version,
    /// 发布的版本更新控制
    force:ForceCtrl,
    ///发布的版本的说明
    #[builder(default)]
    description:Option<String>,
    ///该版本发布的平台
    platform: Platform,
    /// 该版本的可用下载源
    #[builder(via_mutators)]
    download_source:Vec<DownloadSourceItem>
}