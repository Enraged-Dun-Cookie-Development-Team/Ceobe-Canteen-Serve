use std::task::Poll;

use super::{ActorTree, ToTree};
use crate::utils::ready::Ready;

impl<K, T> tower::Service<K> for ActorTree<T>
where
    T: ToTree,
    K: AsRef<[u8]>,
{
    type Error = sled::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;
    type Response = Option<sled::IVec>;

    fn poll_ready(
        &mut self, _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, key: K) -> Self::Future {
        let res = self.0.to_tree().get(key);
        Ready(res)
    }
}
