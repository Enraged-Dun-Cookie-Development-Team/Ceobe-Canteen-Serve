#![feature(type_alias_impl_trait)]

use std::sync::Arc;

use axum::{handler::Handler, Extension, Router};
use bootstrap::create::create_default_user;
use configs::{
    http_listen_config::HttpConfig, GlobalConfig, CONFIG_FILE_JSON,
    CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use figment::providers::{Format, Json, Toml, Yaml};
use mongo_migration::mongo_connection::MongoConnectBuilder;
use orm_migrate::{
    sql_connection::{connect_to_sql_database, get_sql_database},
    Migrator, MigratorTrait,
};
use tokio::sync::oneshot;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    trace::TraceLayer,
};
use utils::user_authorize;

use crate::error::{not_exist, serve_panic};

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

    // 日志配置
    config.logger.register_logger();
    // resp 配置
    resp_result::set_config(&config.resp_result);
    // 鉴权配置
    user_authorize::set_auth_config(&config.user_auth);
    task(config).await
}

async fn task(config: GlobalConfig) {
    // connect to database 连接到数据库
    connect_to_sql_database(&config.database)
        .await
        .expect("无法连接到数据库");
    let db = get_sql_database();
    Migrator::up(db, None)
        .await
        .expect("Migration Sql 数据库失败");
    log::info!("完成对Mysql数据库进行migration操作");
    create_default_user(&config.admin_user).await;

    // mongo db
    MongoConnectBuilder::new(&config.mongodb)
        .await
        .expect("连接到MongoDb数据库异常")
        .apply_mongo_migration(mongo_migration::Migrator)
        .await
        .expect("注册Collection错误")
        .build();

    // load server socket config
    let http_socket = HttpConfig::socket(&config.http_listen);

    let router = Router::new()
        .nest("/api/v1", router::root_route())
        .fallback(not_exist.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(CatchPanicLayer::custom(serve_panic))
                .layer(Extension(Arc::new(config)))
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()),
        );

    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(_) => {
                log::info!("收到退出信号");
                tx.send(())
            }
            Err(err) => {
                log::error!("等待退出信号异常 {err}");
                tx.send(())
            }
        }
    });

    axum::Server::bind(&http_socket)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
        })
        .await
        .expect("服务出现异常");
}
