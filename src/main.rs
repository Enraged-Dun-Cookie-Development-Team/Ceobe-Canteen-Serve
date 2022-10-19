#![feature(type_alias_impl_trait)]

use std::sync::Arc;

use axum::{handler::Handler, Extension, Router};
use axum_starter::ServerPrepare;
use bootstrap::{
    default_user::create_default_user, init::{SqlDatabaseConnect, RespConfig, LoggerRegister, BackAuthConfig, MongoDatabaseConnect, HttpRouterConfig, GracefulExit},
};
use configs::{
    http_listen_config::HttpConfig, GlobalConfig, CONFIG_FILE_JSON,
    CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use database_traits::initial::connect_db_with_migrate;
use figment::providers::{Format, Json, Toml, Yaml};
use mongo_migration::mongo_connection;
use orm_migrate::{sql_connection::SqlDatabase, Migrator, MigratorTrait};
use tokio::sync::oneshot;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    trace::TraceLayer,
};

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

    ServerPrepare::with_config(config)
        .append(LoggerRegister)
        .append(RespConfig)
        .append(BackAuthConfig)
        .append(SqlDatabaseConnect)
        .append(MongoDatabaseConnect)
        .append(HttpRouterConfig)
        .with_global_middleware(CatchPanicLayer::custom(serve_panic))
        .with_global_middleware(TraceLayer::new_for_http())
        .with_global_middleware(CompressionLayer::new())
        .prepare_start()
        .await
        .expect("准备启动服务异常")
        .launch()
        .await
        .expect("启动服务异常");
}

async fn task(config: GlobalConfig) {
    // connect to database 连接到数据库
    connect_db_with_migrate::<SqlDatabase, _, _>(&config.database, |db| {
        async {
            Migrator::up(db, None).await?;
            log::info!("完成对Mysql数据库进行migration操作");
            create_default_user(db, &config.admin_user).await;
            Ok(())
        }
    })
    .await
    .expect("无法连接并初始化SQL数据库");

    // mongo db
    connect_db_with_migrate::<mongo_connection::DatabaseManage, _, _>(
        &config.mongodb,
        mongo_migration::Migrator,
    )
    .await
    .expect("无法连接并初始化MongoDb数据库");

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
