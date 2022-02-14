mod ceobo_actor;
pub mod fut_utils;
mod models;
mod updater_loader;
mod ws_actor;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub const WS_SERVICE: &str = "ws://81.68.101.79:5683/";

pub mod ws {
    pub use crate::ws_actor::{strat_ws, CeoboWebsocket};
}

#[cfg(test)]
mod test {
    use actix::System;

    use crate::{ws, WS_SERVICE};
    #[test]
    fn test_ws() {
        let mut sys = System::new("name");

        sys.block_on(async move {
            let (_res, (_ws, updater)) = ws::strat_ws(WS_SERVICE).await;

            let res = updater.lazy_load(0, &[]).await.unwrap();

            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        });
    }
}
