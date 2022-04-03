use std::ops::Deref;

use super::{db::SledDb, tree::SledTree};

mod load;
mod save;

pub trait ToTree {
    fn to_tree(&self) -> &sled::Tree;
}

pub struct ActorTree<T>(pub(crate) T);

impl<T> ActorTree<T> {
    pub fn new(tree: T) -> Self { Self(tree) }
}

impl ToTree for SledDb {
    fn to_tree(&self) -> &sled::Tree { self.0.deref() }
}

impl ToTree for SledTree {
    fn to_tree(&self) -> &sled::Tree { &self.0 }
}

pub use save::{SavePair, SavePairError, SavePairSer};
