use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SledTree(pub(crate) sled::Tree);

impl Deref for SledTree {
    type Target = sled::Tree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
