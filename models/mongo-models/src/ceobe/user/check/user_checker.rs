use checker::prefabs::collect_checkers::iter_checkers::IntoIterChecker;
use checker::prefabs::no_check::NoCheck;
use mongodb::bson::Uuid;
use range_limit::limits::max_limit::MaxRangeLimit;
use serde::Deserialize;
use crate::ceobe::user::check::CheckError;
use crate::ceobe::user::models::UserChecked;

#[checker::check_gen(
    uncheck = UserUncheck,
    checked = UserChecked,
    error = CheckError
)]
#[derive(Debug, Deserialize)]
pub struct UserChecker {
    mod_id: MaxRangeLimit<String, 16>,
    datasource_push:IntoIterChecker<Vec<Uuid>, NoCheck<Uuid>, Vec<Uuid>>,
}
