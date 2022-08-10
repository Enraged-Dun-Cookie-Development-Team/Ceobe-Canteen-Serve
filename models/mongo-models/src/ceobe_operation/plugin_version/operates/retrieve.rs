// use mongodb::{bson::doc, options::FindOptions};

// use crate::ceobe_operation::plugin_version::models::{Version, PluginVersion};

// use super::{PluginDbOperation, OperateResult, get_plugin_version_collection, OperateError};

// impl PluginDbOperation {
//     pub async fn get_plugin_version_info_by_version(
//         version: Version
//     ) -> OperateResult<PluginVersion> {
//         let collection = get_plugin_version_collection()?;
//         let filter = doc! {
//             "version" : [version.0, version.1, version.2]
//         };
//         collection
//             .doing(|collection| collection.find_one(filter, None))
//             .await?
//             .ok_or(OperateError::VersionNotFind(version))
//     }

//     pub async fn get_newest_plugin_version_info(
//     ) -> OperateResult<PluginVersion> {
//         let collection = get_plugin_version_collection()?;
//         collection
//             .doing(|collection| 
//                 collection.find_one(
//                     None, 
//                     FindOptions::builder()
//                     .projection(doc! {"version":1i32})
//                     .sort(doc! {"version.0":1, "version.1":1, "version.2":1})
//                     .build()
//                 )
//             )
//             .await?
//             .ok_or(OperateError::VersionInfoNoExist)
//     }
// }