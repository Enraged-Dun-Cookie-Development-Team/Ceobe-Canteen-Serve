use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// 下载源对应的平台
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReleasePlatform {
    /// 插件端
    Plugin,
    /// 桌面端
    Desktop,
    /// 口袋端（Android，IOS，WP等）
    Pocket,
}

impl Display for ReleasePlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleasePlatform::Plugin => {
                write!(f, "plugin")
            }
            ReleasePlatform::Desktop => {
                write!(f, "desktop")
            }
            ReleasePlatform::Pocket => {
                write!(f, "pocket")
            }
        }
    }
}

/// 当前下载源支持的平台
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SupportPlatform {
    // desktop
    /// linux发行版桌面端，能兼容X11和Wayland
    Linux,
    MacOS,
    Windows,
    // plugin
    Chrome,
    Firefox,
    Edge,
    /// Webkit, Linux平台上基于webkit4gtk-dev等开发的轻量化浏览器
    Webkit,
    /// MacOS\IPadOS\IOS的内置浏览器
    Safari,
    /// 微软的Internet Explorer 浏览器
    IE,
    // pocket
    Android,
    Ios,
    WindowsPhone,
    /// 鸿蒙系统
    Harmony,
}
