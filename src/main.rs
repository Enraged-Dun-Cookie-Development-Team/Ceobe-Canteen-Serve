#![feature(type_alias_impl_trait)]
use axum_starter::ServerPrepare;

use bootstrap::init::{component_init::{LoggerInitialization, RResultConfig, BackAuthConfig}, db_init::{MysqlDbConnect, MongoDbConnect}, service_init::{RouterFallback, RouteV1, graceful_shutdown}};
use configs::{
    GlobalConfig, CONFIG_FILE_JSON, CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use figment::providers::{Format, Json, Toml, Yaml};
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    trace::TraceLayer,
};

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

#[tokio::main]
async fn main() {
    let config: GlobalConfig = figment::Figment::new()
        .merge(Toml::file(CONFIG_FILE_TOML))
        .merge(Json::file(CONFIG_FILE_JSON))
        .merge(Yaml::file(CONFIG_FILE_YAML))
        .extract()
        .expect("配置文件解析失败");

    ServerPrepare::with_config(config)
        .append(LoggerInitialization)
        .append(RResultConfig)
        .append(BackAuthConfig)
        .append(MysqlDbConnect)
        .append(MongoDbConnect)
        .append(RouteV1)
        .append(RouterFallback)
        .with_global_middleware(CatchPanicLayer::custom(serve_panic))
        .with_global_middleware(TraceLayer::new_for_http())
        .with_global_middleware(CompressionLayer::new())
        .append_fn(graceful_shutdown)
        .prepare_start()
        .await
        .expect("准备启动服务异常")
        .launch()
        .await
        .expect("启动服务异常");
}
