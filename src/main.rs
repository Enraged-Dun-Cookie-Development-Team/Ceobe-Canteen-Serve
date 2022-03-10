#![feature(type_alias_impl_trait)]

use actix_web::{web::{Data, self}, App, HttpServer, HttpRequest};

use error::{GlobalError, not_exist};
use rresult::RResult;
use serves::{CeobeController, MansionController};
use utils::middleware::benchmark::BenchMarkFactor;

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
    logger::init_std(logger::LoggerConfig::default().set_filter(log::LevelFilter::Info))
        .expect("Can not Start Logger");
    task().await
}

async fn task() -> Result<(), crate::error::GlobalError> {
    let (_resp, (updater, sender)) = ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE).await;
    let updater = Data::from(updater);
    let sender = Data::from(sender);
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(BenchMarkFactor)
            .app_data(updater.clone())
            .app_data(sender.clone())
            .service(RootController)
            .default_service(web::to(not_exist))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}
