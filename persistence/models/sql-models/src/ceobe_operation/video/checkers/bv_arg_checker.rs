use checker::check_gen;
use typed_builder::TypedBuilder;

use super::{
    CheckError,
    bv::{Bv, BvChecker},
};

#[derive(Debug, TypedBuilder)]
pub struct BvQuery {
    pub bv: Bv,
}

#[check_gen(
    uncheck = BvQueryUncheck,
    checked = BvQuery,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct BvQueryChecker {
    #[serde(alias = "bv_number")]
    bv: BvChecker,
}
