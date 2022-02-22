mod ceobo_actor;
mod fut_utils;
mod models;
mod updater_loader;
mod ws_actor;

pub const WS_SERVICE: &str = "ws://81.68.101.79:5683/";
pub use updater_loader::{LazyLoad, UpdateLoader};
pub mod ws {
    pub use crate::ws_actor::{start_ws, CeoboWebsocket};
}

#[cfg(test)]
mod test {
    use actix::System;

    use crate::{ws, WS_SERVICE};
    #[test]
    fn test_ws() {
        let mut sys = System::new("test");

        sys.block_on(async move {
            let (_res, updater) = ws::start_ws(WS_SERVICE).await;

            let res = updater.lazy_load(0, &[]).await.unwrap();

            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        });
    }
}
