use checker::check_gen;
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use typed_builder::TypedBuilder;

use super::CheckError;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

#[check_gen(
    uncheck = UsernameUncheck,
    checked =  Username,
    error = CheckError
)]
#[derive(serde::Deserialize, Debug)]
pub struct UsernameChecker {
    pub username: MaxLimitString<16>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Username {
    pub username: String,
}
