use once_cell::sync::Lazy;

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
