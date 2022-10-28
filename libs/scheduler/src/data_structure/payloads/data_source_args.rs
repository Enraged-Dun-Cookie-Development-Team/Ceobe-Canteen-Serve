use std::ops::Deref;

use ahash::AHashMap;
use serde::Serialize;

use super::{ParamName, StrArgument};


#[derive(Debug,Serialize)]
#[serde(untagged)]
pub enum DataSourceArg {
    String(StrArgument),
    Bool(bool)
}

impl From<bool> for  DataSourceArg {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&str> for DataSourceArg {
    fn from(value: &str) -> Self {
        Self::String(StrArgument::from(value.deref()))
    }
}



pub type DataSourceArgs = AHashMap<ParamName,DataSourceArg>;