#![feature(type_alias_impl_trait)]

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use bootstrap::create::create_default_user;
use checker::check_obj;
use configs::{
    http_listen_config::HttpConfig, GlobalConfig, CONFIG_FILE_JSON,
    CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use error::{not_exist, GlobalError};
use figment::providers::{Format, Json, Toml, Yaml};
use mongo_migration::mongo_connection::MongoConnectBuilder;
use orm_migrate::sql_connection::connect_to_sql_database;
use utils::{middleware::benchmark::BenchMarkFactor, user_authorize};

mod bootstrap;
mod configs;
mod error;
mod models;
mod router;
mod serves;
mod utils;

extern crate serde;

#[actix_web::main]
async fn main() -> Result<(), GlobalError> {
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

async fn task(config: GlobalConfig) -> Result<(), crate::error::GlobalError> {
    // connect to database 连接到数据库
    connect_to_sql_database(&config.database)
        .await
        .expect("无法连接到数据库");

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
    // 配置文件打包
    let data_config = Data::new(config);
    HttpServer::new(move || {
        App::new()
            // mid ware 中间件
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(BenchMarkFactor)
            // 配置信息
            .app_data(data_config.clone())
            // 服务
            .service(router::root_route())
            .default_service(web::to(not_exist))
    })
    .bind(http_socket)?
    .run()
    .await?;
    Ok(())
}
