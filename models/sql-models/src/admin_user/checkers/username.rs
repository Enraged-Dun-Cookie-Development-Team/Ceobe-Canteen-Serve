use checker::{check_gen, prefabs::str_len_checker::StrMaxCharLenChecker};
use typed_builder::TypedBuilder;

use super::CheckError;

#[check_gen(
    uncheck = UsernameUncheck,
    checked =  Username,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct UsernameChecker {
    pub username: StrMaxCharLenChecker<String, 16>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Username {
    pub username: String,
}
