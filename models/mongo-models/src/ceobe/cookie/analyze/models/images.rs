use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
pub struct CookieImages {
    pub origin_url: String,
    pub compress_url: Option<String>,
}
