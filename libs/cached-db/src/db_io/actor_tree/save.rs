use std::{future::Future, task::Poll};

use sled::IVec;

use super::{ActorTree, ToTree};
use crate::utils::ready::Ready;

pub struct SavePair<K, V>(K, V);

impl<K, V> SavePair<K, V> {
    pub fn new(key: K, value: V) -> Self { Self(key, value) }
}

impl<K, V, T> tower::Service<SavePair<K, V>> for ActorTree<T>
where
    T: ToTree,
    V: Into<sled::IVec>,
    K: AsRef<[u8]>,
{
    type Error = sled::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;
    type Response = Option<IVec>;

    fn poll_ready(
        &mut self, _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, SavePair(k, v): SavePair<K, V>) -> Self::Future {
        let res = self.0.to_tree().insert(k, v.into());
        Ready(res)
    }
}

#[derive(Debug)]
pub enum SavePairError<Ke, Ve> {
    Key(Ke),
    Value(Ve),
}

pub struct SavePairSer<Ks, Vs>(pub(crate) Ks, pub(crate) Vs);

impl<K, V, Ks, Vs, Kr, Vr> tower::Service<(Kr, Vr)> for SavePairSer<Ks, Vs>
where
    Ks: tower::Service<Kr, Response = K>,
    Vs: tower::Service<Vr, Response = V>,
{
    type Error = SavePairError<Ks::Error, Vs::Error>;
    type Response = SavePair<K, V>;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self, cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        match self.0.poll_ready(cx) {
            Poll::Ready(res) => {
                match res.map_err(SavePairError::Key) {
                    Ok(_) => {
                        self.1.poll_ready(cx).map_err(SavePairError::Value)
                    }
                    err => Poll::Ready(err),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, (k, v): (Kr, Vr)) -> Self::Future {
        let k_task = self.0.call(k);
        let v_task = self.1.call(v);

        async move {
            let k = k_task.await.map_err(SavePairError::Key)?;
            let v = v_task.await.map_err(SavePairError::Value)?;
            Ok(SavePair::new(k, v))
        }
    }
}
