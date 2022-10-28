use core::iter::IntoIterator;
use std::collections::HashMap;

use serde::Serialize;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;

use super::{DataSourceArg, DataSourceArgs, ParamName, SourceType};
#[derive(Debug, Serialize, TypedBuilder)]
pub struct DataSource {
    #[serde(rename = "type")]
    #[builder(setter(transform =|value:impl Into<SourceType>|Into::into(value)))]
    ty: SourceType,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[builder(default,
        setter(
            transform =|value:impl IntoIterator<Item = (impl Into<ParamName>, DataSourceArg)>|
            value.into_iter().map(|(k,v)|(k.into(),v.into())).collect()
        )
    )]
    arg: DataSourceArgs,
}

pub type DataSourceList = SmallVec<[DataSource; 8]>;
