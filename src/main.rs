use std::sync::Arc;

use actix_web::{App, HttpServer};

use ceobe_push::controllers::CeobeController;
use error::GolbalError;

use tokio::runtime;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

extern crate serde;

generate_controller!(RootController, "/", CeobeController);

fn main() -> Result<(), GolbalError> {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() -> Result<(), crate::error::GolbalError> {
    let (_resp, updater) = ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE).await;
    let updater = Arc::new(updater);
    HttpServer::new(move || App::new().data(updater.clone()).service(RootController))
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
    Ok(())
}
