use std::time::Duration;

use serde::Serialize;
use smallvec::SmallVec;
use typed_builder::TypedBuilder;

use super::{DataSourceList, GroupName, DataSource};

#[derive(Debug, Serialize, TypedBuilder)]
pub struct Group {
    #[builder(default, setter(transform = |name: impl Into<GroupName>|Some(Into::into(name))))]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<GroupName>,
    #[serde(rename = "datasource")]
    #[serde(skip_serializing_if = "SmallVec::is_empty")]
    #[builder(
        default, 
        setter(
            transform = 
            |list: impl IntoIterator<Item = DataSource>|
            list.into_iter().collect()
        )
    )]
    data_source: DataSourceList,
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |interval: Duration| Some(interval.as_millis())))]
    interval_milliseconds : Option<u128>,
}

pub type Groups = SmallVec<[Group; 8]>;
