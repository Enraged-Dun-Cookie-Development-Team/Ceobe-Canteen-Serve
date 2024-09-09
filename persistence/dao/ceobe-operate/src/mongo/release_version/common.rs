use serde::Serialize;
use db_ops_prelude::mongo_models::ceobe::operation::version::models::{ReleasePlatform, Version};
use db_ops_prelude::mongodb::bson::{doc, Document, to_bson};

pub(super) fn generate_set_document<T: Serialize>(
    key: &str, value: T,
) -> super::Result<Document> {
    let doc = doc! {
      key:{
            "$set" : to_bson(&value)?
        }
    };

    Ok(doc)
}

pub(super) fn generate_release_version_filter(
    version: &Version, release_platform: &ReleasePlatform,
) -> super::Result<Document> {
    let doc = doc! {
        "version" : to_bson(version)?,
        "platform": to_bson(release_platform)?
    };
    Ok(doc)
}
