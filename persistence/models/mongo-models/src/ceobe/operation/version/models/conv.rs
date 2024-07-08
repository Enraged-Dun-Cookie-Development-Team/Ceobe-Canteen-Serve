use semver::Version;
use sql_models::ceobe_operation::{app_version, desktop_version};
use tracing_unwrap::ResultExt;

use crate::ceobe::operation::{
    plugin_version::{self, DownloadResource, SpareLink},
    version::models::{
        DownloadSourceItem, ForceCtrl, Primary,
        ReleasePlatform::{Desktop, Plugin, Pocket},
        ReleaseVersion, ResourceUrl, SupportPlatform,
        SupportPlatform::{Android, MacOS, Windows},
    },
};

impl From<plugin_version::Checked> for ReleaseVersion {
    fn from(
        plugin_version::Checked {
            version:
                checker::prefabs::version_checker::Version {
                    major,
                    minor,
                    security,
                },
            description,
            down:
                DownloadResource {
                    crx,
                    spare_crx,
                    zip,
                    spare_zip,
                    chrome,
                    edge,
                    firefox,
                    spare: SpareLink { url, msg },
                },
            ..
        }: plugin_version::Checked,
    ) -> Self {
        ReleaseVersion::builder()
            .version(Version::new(major as _, minor as _, security as _))
            .description(description)
            .platform(Plugin)
            .force(
                ForceCtrl::builder()
                    .force_update()
                    .previous_force_version(Version::new(
                        major as _,
                        minor as _,
                        security as _,
                    ))
                    .build(),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("CRX")
                    .primary_url(
                        ResourceUrl::builder().name(Primary).url(crx).build(),
                    )
                    .extend_spare_url(spare_crx.map(|url| {
                        ResourceUrl::builder()
                            .name("CRX备用")
                            .url(url)
                            .manual()
                            .build()
                    }))
                    .build(),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("ZIP")
                    .primary_url(
                        ResourceUrl::builder().name(Primary).url(zip).build(),
                    )
                    .extend_spare_url(spare_zip.map(|url| {
                        ResourceUrl::builder()
                            .name("Zip备用")
                            .url(url)
                            .manual()
                            .build()
                    }))
                    .build(),
            )
            .extend_download_source(
                ["Chrome", "Edge", "FireFox"]
                    .into_iter()
                    .zip([chrome, edge, firefox])
                    .map(|(name, url)| {
                        DownloadSourceItem::builder()
                            .name(name)
                            .primary_url(
                                ResourceUrl::builder()
                                    .name(Primary)
                                    .url(url)
                                    .build(),
                            )
                            .build()
                    }),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("云盘备用")
                    .description(msg)
                    .primary_url(
                        ResourceUrl::builder()
                            .name(Primary)
                            .url(url)
                            .manual()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

impl From<app_version::Checked> for ReleaseVersion {
    fn from(
        app_version::Checked {
            version,
            force,
            last_force_version,
            description,
            apk,
            spare_apk,
            baidu,
            baidu_text,
        }: app_version::Checked,
    ) -> Self {
        ReleaseVersion::builder()
            // 经过校验的version,没有问题
            .version(version.parse().unwrap_or_log())
            .force(
                ForceCtrl::builder()
                    .set_force_update(force)
                    .previous_force_version(
                        last_force_version.parse().unwrap_or_log(),
                    )
                    .build(),
            )
            .platform(Pocket)
            .description(description)
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("安卓下载")
                    .primary_url(
                        ResourceUrl::builder().name(Primary).url(apk).build(),
                    )
                    .add_spare_url(
                        ResourceUrl::builder()
                            .name("安卓备用下载")
                            .url(spare_apk)
                            .manual()
                            .add_support_platform(Android)
                            .build(),
                    )
                    .build(),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("百度云盘下载")
                    .description(baidu_text)
                    .primary_url(
                        ResourceUrl::builder()
                            .name(Primary)
                            .url(baidu)
                            .manual()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}

impl From<desktop_version::Checked> for ReleaseVersion {
    fn from(
        desktop_version::Checked {
            version,
            force,
            last_force_version,
            description,
            exe,
            spare_exe,
            dmg,
            spare_dmg,
            baidu,
            baidu_text,
        }: desktop_version::Checked,
    ) -> Self {
        ReleaseVersion::builder()
            // 经过校验的version,没有问题
            .version(version.parse().unwrap_or_log())
            .force(
                ForceCtrl::builder()
                    .set_force_update(force)
                    .previous_force_version(
                        last_force_version.parse().unwrap_or_log(),
                    )
                    .build(),
            )
            .platform(Desktop)
            .description(description)
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("Windows安装包下载(.exe)")
                    .primary_url(
                        ResourceUrl::builder()
                            .name(Primary)
                            .url(exe)
                            .add_support_platform(Windows)
                            .build(),
                    )
                    .add_spare_url(
                        ResourceUrl::builder()
                            .name("Windows安装包备用下载")
                            .url(spare_exe)
                            .manual()
                            .add_support_platform(Windows)
                            .build(),
                    )
                    .build(),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("MacOS安装包下载(.dmg)")
                    .primary_url(
                        ResourceUrl::builder()
                            .name(Primary)
                            .url(dmg)
                            .add_support_platform(MacOS)
                            .build(),
                    )
                    .add_spare_url(
                        ResourceUrl::builder()
                            .name("MacOS安装包备用下载")
                            .url(spare_dmg)
                            .manual()
                            .add_support_platform(MacOS)
                            .build(),
                    )
                    .build(),
            )
            .add_download_source(
                DownloadSourceItem::builder()
                    .name("百度云盘下载")
                    .description(baidu_text)
                    .primary_url(
                        ResourceUrl::builder()
                            .name(Primary)
                            .url(baidu)
                            .manual()
                            .build(),
                    )
                    .build(),
            )
            .build()
    }
}
