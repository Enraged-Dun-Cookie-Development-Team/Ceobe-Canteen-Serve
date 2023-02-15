use checker::{
    prefabs::{
        collect_checkers::iter_checkers::IntoIterChecker, no_check::NoCheck,
    },
    ToCheckRequire,
};
use mongodb::bson::Uuid;
use range_limit::limits::max_limit::MaxRangeLimit;
use serde::Deserialize;
use typed_builder::TypedBuilder;

use crate::ceobe::user::{check::CheckError, models::UserChecked};

#[checker::check_gen(
    uncheck = UserUncheck,
    checked = UserChecked,
    error = CheckError
)]
#[derive(Debug, Deserialize, TypedBuilder)]
pub struct UserChecker {
    #[builder(setter(
        transform = |id:String| ToCheckRequire::require_check(id)
    ))]
    mob_id: MaxRangeLimit<String, 16>,
    #[builder(setter(
        transform = |datasource:Vec<Uuid>| ToCheckRequire::require_check(datasource)
    ))]
    datasource_push: IntoIterChecker<Vec<Uuid>, NoCheck<Uuid>, Vec<Uuid>>,
}
