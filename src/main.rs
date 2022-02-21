use std::sync::Arc;

use actix_web::{App, HttpServer};

use ceobe_push::controllers::CeobeController;
use error::GlobalError;

mod ceobe_push;
mod database;
mod error;
mod mansion;
mod utils;

extern crate serde;

generate_controller!(RootController, "/", CeobeController);

#[actix_web::main]
async fn main() -> Result<(), GlobalError> {
    task().await
}

async fn task() -> Result<(), crate::error::GlobalError> {
    let (_resp, updater) = ceobe_manager::ws::start_ws(ceobe_manager::WS_SERVICE).await;
    let updater = Arc::new(updater);
    HttpServer::new(move || App::new().data(updater.clone()).service(RootController))
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
    Ok(())
}
