use db_ops_prelude::{
    mongo_models::ceobe::operation::version::models::{
        ReleasePlatform, Version,
    },
    mongodb::bson::{doc, to_bson, Document},
};

pub(super) fn generate_release_version_filter(
    version: &Version, release_platform: &ReleasePlatform,
) -> super::Result<Document> {
    let doc = doc! {
        "version" : to_bson(version)?,
        "platform": to_bson(release_platform)?
    };
    Ok(doc)
}
