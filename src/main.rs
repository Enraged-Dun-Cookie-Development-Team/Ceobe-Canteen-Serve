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
use database::ServeDatabase;
use error::{not_exist, GlobalError};
use figment::providers::{Format, Json, Toml, Yaml};
use time_usage::async_time_usage_with_name;
use utils::{
    middleware::benchmark::BenchMarkFactor,
    mongodb_utils::mongo_build::MongoBuild, user_authorize,
};

mod bootstrap;
mod configs;
mod database;
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
    // connect to ceobe websocket
    let (_resp, (updater, sender)) = async_time_usage_with_name(
        "连接到小刻蹲饼",
        ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE),
    )
    .await;
    let updater = Data::from(updater);
    let sender = Data::from(sender);
    // connect to database 连接到数据库
    let db_conn = ServeDatabase::connect(&config.database)
        .await
        .expect("无法连接到数据库")
        .register_models()
        .await
        .expect("无法在数据库中创建实体");
    create_default_user(&config.admin_user, &db_conn).await;
    let db_data = Data::new(db_conn);
    // mongo db
    let mongo_conn = async_time_usage_with_name(
        "连接到MongoDb数据库",
        MongoBuild::with_config(&config.mongodb)
            .await
            .expect("无法连接到MongoDb")
            .collect_migration(mongo_migration::Migrator),
    )
    .await
    .expect("Mongo Db 模型建立失败")
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
            // 管理的全局事务
            // ceobe ws 通讯
            .app_data(updater.clone())
            .app_data(sender.clone())
            // 数据库连接
            .app_data(db_data.clone())
            .app_data(mongo_conn.clone())
            // 配置信息
            .app_data(data_config.clone())
            // 服务
            // .service(RootController)
            .service(router::root_route())
            .default_service(web::to(not_exist))
    })
    .bind(http_socket)?
    .run()
    .await?;
    Ok(())
}
