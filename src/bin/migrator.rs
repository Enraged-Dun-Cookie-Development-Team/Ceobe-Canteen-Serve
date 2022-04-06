use std::env;


use orm_migrate::{async_std, Migrator, MigratorTrait};

#[async_std::main]
async fn main() {
    logger::init_std(
        logger::LoggerConfig::default().set_filter(log::LevelFilter::Info),
    )
    .expect("无法启动日志系统");

    let url = env::args().nth(1).expect("请启动时提供数据库URL");
    log::info!("取得数据库URL {}", url);

    log::info!("开始连接数据库，准备建表");

    let db = sea_orm::Database::connect(url)
        .await
        .expect("无法连接到数据库");

    log::info!("成功连接到数据库，开始数据库建表");

    let _resp = Migrator::up(&db, None).await.expect("数据库建表失败");

    log::info!("完成建表");
}
