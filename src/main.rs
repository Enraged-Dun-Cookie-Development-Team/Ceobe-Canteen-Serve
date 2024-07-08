extern crate serde;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use axum::routing::Route;
use axum_starter::ServerPrepare;
use bootstrap::{
    init::{
        component_init::{BackendAuthConfig, RResultConfig},
        db_init::{MongoDbConnect, MysqlDbConnect, RedisDbConnect},
        service_init::{graceful_shutdown, RouteV1, RouterFallback},
    },
    middleware::{
        cors::PrepareCors, panic_report::PrepareCatchPanic,
        tracing_request::PrepareRequestTracker,
    },
};
use ceobe_qiniu_upload::QiniuUpload;
use configs::{
    auth_config::AuthConfig, cors_config::CorsConfigImpl,
    mob_config::MobPushConfig, qiniu_secret::QiniuUploadConfig,
    qq_channel::QqChannelConfig, resp_result_config::RespResultConfig,
    schedule_notifier_config::ScheduleNotifierConfig,
    tc_cloud_config::TcCloudConfig, GlobalConfig, CONFIG_FILE_JSON,
    CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use figment::providers::{Env, Format, Json, Toml, Yaml};
use general_request_client::axum_starter::RequestClientPrepare;
use mob_push_server::axum_starter::MobPushPrepare;
use qq_channel_warning::QqChannelPrepare;
use request_clients::bili_client::BiliClientPrepare;
use scheduler_notifier::axum_starter::ScheduleNotifierPrepare;
use tencent_cloud_server::axum_starter::TencentCloudPrepare;
use tower_http::compression::CompressionLayer;
use tracing_unwrap::ResultExt;

use crate::bootstrap::decorator::Decroator;
use crate::bootstrap::postprepare::migrate_version;

mod bootstrap;
mod configs;
mod error;
mod middleware;

mod router;
mod serves;
mod utils;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
        .merge(Env::prefixed("CEOBE_").split("__"))
        .extract()
        .expect("配置文件解析失败");

    ServerPrepare::with_config(config)
        .init_logger()
        .expect("日志初始化失败")
        .convert_state()
        .prepare_decorator(Decroator)
        // components
        .prepare(RResultConfig::<_, RespResultConfig>)
        .prepare(BackendAuthConfig::<_, AuthConfig>)
        .prepare_state(RequestClientPrepare)
        .prepare_state(QiniuUpload::<_, QiniuUploadConfig>)
        .prepare_state(BiliClientPrepare)
        .prepare_state(ScheduleNotifierPrepare::<_, ScheduleNotifierConfig>)
        .prepare_state(MobPushPrepare::<_, MobPushConfig>)
        .prepare_state(QqChannelPrepare::<_, QqChannelConfig>)
        .prepare_state(TencentCloudPrepare::<_, TcCloudConfig>)
        // database
        .prepare_concurrent(|set| {
            set.join_state(MysqlDbConnect)
                .join_state(MongoDbConnect)
                .join_state(RedisDbConnect)
        })
        // router
        .prepare_route(RouteV1)
        .prepare_route(RouterFallback)
        .prepare_middleware::<Route, _>(PrepareCors::<_, CorsConfigImpl>)
        .prepare_middleware::<Route, _>(
            PrepareCatchPanic::<_, QqChannelConfig>,
        )
        .layer(CompressionLayer::new())
        .prepare_middleware::<Route, _>(PrepareRequestTracker)
        .graceful_shutdown(graceful_shutdown())
        .post_prepare(migrate_version)
        .preparing()
        
        .await
        .expect("准备启动服务异常")
        .launch()
        .await
        .expect("启动服务异常");
}
