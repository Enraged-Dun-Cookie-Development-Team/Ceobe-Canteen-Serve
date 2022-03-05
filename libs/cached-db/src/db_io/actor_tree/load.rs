use std::task::Poll;

use crate::utils::ready::Ready;

use super::{ActorTree, ToTree};

impl<K, T> tower::Service<K> for ActorTree<T>
where
    T: ToTree,
    K: AsRef<[u8]>,
{
    type Response = Option<sled::IVec>;

    type Error = sled::Error;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, key: K) -> Self::Future {
        let res = self.0.to_tree().get(key);
        Ready(res)
    }
}
