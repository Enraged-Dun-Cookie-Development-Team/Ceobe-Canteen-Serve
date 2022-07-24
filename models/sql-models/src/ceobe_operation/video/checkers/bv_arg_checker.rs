use checker::check_obj;
use typed_builder::TypedBuilder;

use super::{
    bv::{Bv, BvChecker},
    CheckError,
};

#[derive(Debug, TypedBuilder)]
pub struct BvQuery {
    pub bv: Bv,
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct BvQueryUncheck = BvQueryChecker > BvQuery{
        bv :BvChecker
    }
    err : CheckError
}
