use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    vis = "pub",
    name = "ComicInfoWithoutCid",
    extra(derive(Debug, Clone, Serialize, Deserialize, TypedBuilder))
))]
pub struct TerraComicModel {
    #[sub_model(ignore("ComicInfoWithoutCid"))]
    pub cid: String,
    pub cover: String,
    pub introduction: String,
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub subtitle: String,
    pub title: String,
}
