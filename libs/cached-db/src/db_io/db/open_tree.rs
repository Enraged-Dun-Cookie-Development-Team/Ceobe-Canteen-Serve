use std::{future::Future, task::Poll};

use super::SledDb;
use crate::db_io::tree::SledTree;

impl<T: AsRef<[u8]>> tower::Service<T> for SledDb {
    type Error = sled::Error;
    type Response = SledTree;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self, _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        let new_tree = self.0.open_tree(req).map(|t| SledTree(t));
        async move { new_tree }
    }
}
