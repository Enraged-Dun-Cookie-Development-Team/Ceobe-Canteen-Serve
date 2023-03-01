mod check;
mod models;

pub use check::{
    plugin_version_checker::PluginVersionChecker as Checker, CheckError,
};
pub use models::{
    DownloadResource, PluginVersion, PluginVersionChecked, SpareLink, Version,
};
pub type Uncheck = checker::Uncheck<Checker>;
pub type Checked = checker::Checked<Checker>;

pub mod version {
    pub use super::check::version_checker::VersionChecker as Checker;
    pub type Uncheck = checker::Uncheck<Checker>;
    pub type Checked = checker::Checked<Checker>;
}