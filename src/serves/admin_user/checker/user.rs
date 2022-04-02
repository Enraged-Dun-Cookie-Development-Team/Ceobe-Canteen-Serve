use crate::{utils::data_struct::MaxLimitString, serves::admin_user::error::AdminUserError};

crate::check_obj! {
    {#[derive(serde::Deserialize,Debug)]}
    {#[derive(serde::Serialize,serde::Deserialize,Debug,Clone)]}
    pub struct UsernameUncheck = UsernameChecker > Username{
        pub username: MaxLimitString<16>
    }
    err:AdminUserError
}