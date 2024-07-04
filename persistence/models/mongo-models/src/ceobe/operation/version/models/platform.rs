use serde::{Deserialize, Serialize};

/// 下载源对应的平台
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    /// 插件端
    Plugin,
    /// 桌面端
    Desktop,
    /// 口袋端（Android，IOS，WP等）
    Pocket,
}
