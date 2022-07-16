use crate::{utils::data_struct::MaxLimitString, serves::backend::user_auth::error::AdminUserError};



crate::check_obj! {
    #[derive(serde::Deserialize,Debug)]
    pub struct UsernameUncheck = UsernameChecker > Username{
        pub username : MaxLimitString<16>
    }
    err:AdminUserError
}


crate::quick_struct!{
    pub Username{
        username:String
    }
}