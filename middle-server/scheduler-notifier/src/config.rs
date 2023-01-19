use url::Url;

/// 调度器 Notifier 配置
pub trait SchedulerNotifierConfig {
    /// base url
    ///
    /// # Note
    /// 该base url 的 path 部分会被忽略
    fn base_url(&self) -> Url;
}
