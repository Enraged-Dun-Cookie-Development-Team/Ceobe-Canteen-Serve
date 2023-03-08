use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;


#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(
    none(
        vis = "pub",
        name = "SingleData",
        extra(derive(Debug, Clone, TypedBuilder))
    )
)]
pub struct TempListModel {
    pub platform: String,
    pub source: String,
    pub source_config_id: i32,
    pub time: i32,
    #[sub_model(want("SingleData"))]
    pub data: String,
}