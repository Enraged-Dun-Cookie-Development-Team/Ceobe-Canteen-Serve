use checker::{check_obj, prefabs::no_check::NoCheck};
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::ceobe_operation::resource::models::model_resource;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOperationResource {
    _place_hold: i32,
}

#[check_obj(
    uncheck = CeobeOperationResourceUncheck,
    checked = CeobeOperationResource,
    error = CheckError,
)]
pub struct CeobeOperationResourceChecker {
    _place_hold: NoCheck<i32>,
}

impl model_resource::ActiveModel {}
