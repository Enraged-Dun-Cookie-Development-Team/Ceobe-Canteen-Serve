use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::sql::app_version::models::model_app_version;

// app版本
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AppVersionView {
    pub version: String,
    pub force: bool,
    pub last_force_version: String,
    pub description: String,
}

// app版本转换
impl From<model_app_version::Model> for AppVersionView {
    fn from(
        model_app_version::Model {
            version,
            force,
            last_force_version,
            description,
            ..
        }: model_app_version::Model,
    ) -> Self {
        Self {
            version,
            force,
            last_force_version,
            description,
        }
    }
}

// 插件版本
pub use mongo_migration::mongo_models::ceobe_operation::plugin_version::{
    DownloadView, PluginVersionView,
};
