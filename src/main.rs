#![feature(type_alias_impl_trait)]

use std::sync::Arc;

use actix_web::{App, HttpServer};

use ceobe_push::controllers::CeobeController;
use error::GlobalError;
use utils::middleware::benchmark::BenchMarkFactor;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

extern crate serde;

generate_controller!(RootController, "/api/v0", CeobeController);

#[actix_web::main]
async fn main() -> Result<(), GlobalError> {
    logger::init_std(logger::LoggerConfig::default().set_filter(log::LevelFilter::Info))
        .expect("Can not Start Logger");
    task().await
}

async fn task() -> Result<(), crate::error::GlobalError> {
    let (_resp, updater) = ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE).await;
    let updater = Arc::new(updater);
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(BenchMarkFactor)
            .data(updater.clone())
            .service(RootController)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}
