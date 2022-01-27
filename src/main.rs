use std::net::Ipv4Addr;

use ceobe_push::dao::DataItem;
use database::{config::DbConfig, ServeDatabase};
use error::GolbalError;
use tokio::runtime;
use url::Url;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

#[macro_use]
extern crate serde;

const DUN_BACKEND: &str = "ws://127.0.0.1/";
const PUSH_URL: &str = "http://localhost";
fn main()->Result<(),GolbalError> {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() -> Result<(), crate::error::GolbalError> {
    let _db = ServeDatabase::connet(
        // 这里是临时用法，通常情况下通过配置文件读取配置
        &DbConfig {
        scheme: "mysql".to_string(),
        username: "root".to_string(),
        password: "password".to_string(),
        host: Ipv4Addr::LOCALHOST,
        port: 3306,
        name: "mansion_data".to_string(),
        max_conn: 16,
        min_conn: 2,
        logger: false,
    })
    .await?;

    Ok(())
}
