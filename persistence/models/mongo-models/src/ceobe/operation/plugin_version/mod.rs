pub use checker::prefabs::version_checker::Version;
pub use check::{
    plugin_version_checker::PluginVersionChecker as Checker, CheckError,
};
pub use models::{
    DownloadResource, PluginVersion, PluginVersionChecked, SpareLink,
};

mod check;
mod models;

pub type Uncheck = checker::Uncheck<Checker>;
pub type Checked = checker::Checked<Checker>;

pub mod version {
    use checker::prefabs::version_checker::Version;
    pub use checker::prefabs::version_checker::VersionChecker as Checker;

    pub type Uncheck = checker::Uncheck<Checker<Version>>;
    pub type Checked = checker::Checked<Checker<Version>>;
}
