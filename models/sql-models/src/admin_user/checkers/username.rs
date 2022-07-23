use checker::check_obj;
use range_limit::{limits::max_limit::MaxLimit, RangeBoundLimit};
use typed_builder::TypedBuilder;

use crate::admin_user::AdminUserError;

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;

check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct UsernameUncheck = UsernameChecker > Username{
        pub username : MaxLimitString<16>
    }
    err:AdminUserError
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Username {
    pub username: String,
}
