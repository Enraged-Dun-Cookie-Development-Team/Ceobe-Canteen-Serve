#![clippy::msrv ="1.77.0"]
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
        panic_report::PrepareCatchPanic,
        tracing_request::PrepareRequestTracker,
    },
};
use ceobe_qiniu_upload::QiniuUpload;
use configs::{
    auth_config::AuthConfig, mob_config::MobPushConfig,
    qiniu_secret::QiniuUploadConfig, qq_channel::QqChannelConfig,
    resp_result_config::RespResultConfig,
    schedule_notifier_config::ScheduleNotifierConfig, GlobalConfig,
    CONFIG_FILE_JSON, CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use figment::providers::{Env, Format, Json, Toml, Yaml};
use general_request_client::axum_starter::RequestClientPrepare;
use http::Method;
use mob_push_server::axum_starter::MobPushPrepare;
use qq_channel_warning::QqChannelPrepare;
use request_clients::bili_client::BiliClientPrepare;
use scheduler_notifier::axum_starter::ScheduleNotifierPrepare;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    cors::CorsLayer,
};
use tracing_unwrap::ResultExt;

use crate::error::serve_panic;

mod bootstrap;
mod configs;
mod error;
mod middleware;

mod router;
mod serves;
mod utils;
#[cfg(not(target_env = "msvc"))] use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
        // components
        .prepare(RResultConfig::<_, RespResultConfig>)
        .prepare(BackendAuthConfig::<_, AuthConfig>)
        .prepare_state(RequestClientPrepare)
        .prepare_state(QiniuUpload::<_, QiniuUploadConfig>)
        .prepare_state(BiliClientPrepare)
        .prepare_state(ScheduleNotifierPrepare::<_, ScheduleNotifierConfig>)
        .prepare_state(MobPushPrepare::<_, MobPushConfig>)
        .prepare_state(QqChannelPrepare::<_, QqChannelConfig>)
        // database
        .prepare_concurrent(|set| {
            set.join_state(MysqlDbConnect)
                .join_state(MongoDbConnect)
                .join_state(RedisDbConnect)
        })
        // router
        .prepare_route(RouteV1)
        .prepare_route(RouterFallback)
        .layer(CorsLayer::new().allow_methods([Method::GET]).allow_origin([
            "https://www.ceobecanteen.top".parse().unwrap(),
            "https://ceobecanteen.top".parse().unwrap(),
        ]))
        .prepare_middleware::<Route, _>(
            PrepareCatchPanic::<_, QqChannelConfig>,
        )
        .layer(CatchPanicLayer::custom(serve_panic))
        .layer(CompressionLayer::new())
        .prepare_middleware::<Route, _>(PrepareRequestTracker)
        .graceful_shutdown(graceful_shutdown())
        .convert_state()
        .prepare_start()
        .await
        .expect("准备启动服务异常")
        .launch()
        .await
        .expect("启动服务异常");
}
