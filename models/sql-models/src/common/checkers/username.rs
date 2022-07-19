use checker::check_obj;
use range_limit::{RangeBoundLimit, limits::max_limit::MaxLimit};
use typed_builder::TypedBuilder;

use crate::common::CommonError;

type MaxLimitString<const H: usize> =
    RangeBoundLimit<String, MaxLimit<H>>;


check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct UsernameUncheck = UsernameChecker > Username{
        pub username : MaxLimitString<16>
    }
    err:CommonError
}

#[derive(Debug,Clone,TypedBuilder)]
pub struct Username{
    pub username : String
}