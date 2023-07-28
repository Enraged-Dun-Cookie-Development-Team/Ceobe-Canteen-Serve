use checker::check_gen;
use typed_builder::TypedBuilder;

use super::{
    bv::{Bv, BvChecker},
    CheckError,
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
