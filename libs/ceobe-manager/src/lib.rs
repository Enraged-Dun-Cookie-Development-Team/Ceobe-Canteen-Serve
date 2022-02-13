pub mod fut_utils;
mod ceobo_actor;
mod models;
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

pub mod ws{
    pub use crate::ws_actor::{CeoboWebsocket,strat_ws};
}