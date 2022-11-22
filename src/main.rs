use std::{
    io::{stdout, Write},
    time::Duration,
};

use axum_starter::ServerPrepare;
use bootstrap::{
    init::{
        component_init::{BackendAuthConfig, RResultConfig},
        db_init::{MongoDbConnect, MysqlDbConnect},
        service_init::{graceful_shutdown, RouteV1, RouterFallback},
    },
    midllewares::tracing_request::tracing_request,
};
use ceobe_qiniu_upload::QiniuUpload;
use configs::{
    auth_config::AuthConfig, qiniu_secret::QiniuUploadConfig,
    resp_result_config::RespResultConfig, GlobalConfig, CONFIG_FILE_JSON,
    CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use figment::providers::{Env, Format, Json, Toml, Yaml};
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
};
use tracing_unwrap::ResultExt;

use crate::error::serve_panic;

mod bootstrap;
mod configs;
mod error;
mod middleware;
mod models;
mod router;
mod serves;
mod utils;

extern crate serde;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Init Rt failure");
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    rt.block_on(main_task());
    stdout().flush().expect_or_log("failure to flush stdout");
    std::thread::sleep(Duration::from_millis(500))
}

async fn main_task() {
    let config: GlobalConfig = figment::Figment::new()
        .merge(Toml::file(CONFIG_FILE_TOML))
        .merge(Json::file(CONFIG_FILE_JSON))
        .merge(Yaml::file(CONFIG_FILE_YAML))
        .merge(Env::prefixed("CEOBE_"))
        .extract()
        .expect("配置文件解析失败");

    ServerPrepare::with_config(config)
        .init_logger()
        .expect("日志初始化失败")
        // components
        .append(RResultConfig::<_, RespResultConfig>)
        .append(BackendAuthConfig::<_, AuthConfig>)
        .append(QiniuUpload::<_, QiniuUploadConfig>)
        // database
        .append_concurrent(|set| {
            set.join(MysqlDbConnect).join(MongoDbConnect)
            // .join(RedisDbConnect)
        })
        // router
        .append(RouteV1)
        .append(RouterFallback)
        .with_global_middleware(CatchPanicLayer::custom(serve_panic))
        .with_global_middleware(CompressionLayer::new())
        .with_global_middleware(tracing_request())
        .append_fn(graceful_shutdown)
        .prepare_start()
        .await
        .expect("准备启动服务异常")
        .launch()
        .await
        .expect("启动服务异常");
}
