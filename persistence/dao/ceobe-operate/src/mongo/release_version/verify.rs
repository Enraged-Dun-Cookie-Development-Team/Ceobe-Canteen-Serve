use db_ops_prelude::{
    mongo_connection::CollectionGuard,
    mongo_models::ceobe::operation::version::models::{
        ReleaseVersion, Version,
    },
    mongodb::{
        bson::{doc, to_bson},
        options::FindOneOptions,
    },
};
use serde::{Deserialize, Serialize};

use super::{Error, Result};
pub(super) async fn suitable_version(
    collection: &CollectionGuard<ReleaseVersion>,
    release_version: &ReleaseVersion,
) -> Result<()> {
    #[derive(Debug, Deserialize, Serialize)]
    struct VersionOnly {
        version: Version,
    }

    // 找到当前平台的最新的发布版本，与当前添加版本比较。新版本必须更新
    let filter = doc! {
        "platform":to_bson(&release_version.platform)?
    };
    let exist_latest_version = collection
        .with_mapping::<VersionOnly>()
        .doing(|collection| {
            collection.find_one(
                filter,
                FindOneOptions::builder()
                    .sort(doc! {"$natural": -1i32})
                    .projection(doc! {"version": 1i32})
                    .build(),
            )
        })
        .await?;

    match exist_latest_version {
        Some(VersionOnly { version })
            if release_version.version <= version =>
        {
            Err(Error::VersionTooOld(version, release_version.platform))
        }
        _ => Ok(()),
    }
}
