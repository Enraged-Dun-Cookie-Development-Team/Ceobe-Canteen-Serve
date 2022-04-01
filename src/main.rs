#![feature(type_alias_impl_trait)]

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use configs::{
    GlobalConfig, CONFIG_FILE_JSON, CONFIG_FILE_TOML, CONFIG_FILE_YAML,
};
use database::ServeDatabase;
use error::{not_exist, GlobalError};
use figment::providers::{Format, Json, Toml, Yaml};
use serves::{CeobeController, MansionController};
use utils::{
    http_serve::MongoRegister, middleware::benchmark::BenchMarkFactor,
    mongodb_utils::mongo_build::MongoBuild, user_authorize,
};

mod configs;
mod database;
mod error;
mod serves;
mod utils;

extern crate serde;

generate_controller!(
    RootController,
    "/api/v0",
    CeobeController,
    // database not add yet
    MansionController
);

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
    let (_resp, (updater, sender)) =
        ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE).await;
    let updater = Data::from(updater);
    let sender = Data::from(sender);
    // connect to database 连接到数据库
    let db_conn = ServeDatabase::connect(&config.database)
        .await
        .expect("无法连接到数据库");
    let db_data = Data::new(db_conn);
    // mongo db
    let mongo_conn = MongoBuild::with_config(&config.mongodb)
        .await
        .expect("无法连接到MongoDb")
        .register_collections(MansionController::mongo_register())
        .await
        .build();
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
            .service(RootController)
            .default_service(web::to(not_exist))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}
