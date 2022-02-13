mod fut_utils;
use actix::{fut::wrap_future, Actor, ActorFuture, AsyncContext, Context, SpawnHandle};
use futures_util::Future;

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
