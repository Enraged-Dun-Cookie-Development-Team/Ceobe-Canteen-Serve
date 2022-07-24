use once_cell::sync::Lazy;

use self::error::CeobeOperationVideoError;
use crate::utils::user_authorize::{
    auth_level::prefabs::Chef, AuthenticationLevel,
};

mod controllers;
mod error;
mod view;

static REQUEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) \
             Gecko/20100101 Firefox/102.0",
        )
        .build()
        .expect("Reqwest 客户端创建失败")
});

type VideoAuthentication =
    AuthenticationLevel<Chef, CeobeOperationVideoError>;
