use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    hash::BuildHasher,
};
pub(super) trait GetOrCreate<K, V> {
    fn get_mut_or_create(&mut self, key: K, value: V) -> &mut V;
    fn get_mut_or_default(&mut self, key: K) -> &mut V
    where
        V: Default;
    fn get_mut_or_create_with<F: FnOnce() -> V>(
        &mut self, key: K, default: F,
    ) -> &mut V;
    fn get_or_try_create_with<F: FnOnce() -> Result<V, E>, E>(
        &mut self, key: K, default: F,
    ) -> Result<&mut V, E>;
}

impl<Q: std::hash::Hash + std::cmp::Eq, V, R: BuildHasher> GetOrCreate<Q, V>
    for HashMap<Q, V, R>
{
    fn get_mut_or_create(&mut self, key: Q, value: V) -> &mut V {
        self.entry(key).or_insert(value)
    }

    fn get_mut_or_default(&mut self, key: Q) -> &mut V
    where
        V: Default,
    {
        self.entry(key).or_default()
    }

    fn get_mut_or_create_with<F: FnOnce() -> V>(
        &mut self, key: Q, default: F,
    ) -> &mut V {
        self.entry(key).or_insert_with(default)
    }

    fn get_or_try_create_with<F: FnOnce() -> Result<V, E>, E>(
        &mut self, key: Q, default: F,
    ) -> Result<&mut V, E> {
        match self.entry(key) {
            Occupied(entry) => Ok(entry.into_mut()),
            Vacant(entry) => {
                let v = default()?;
                let v = entry.insert(v);
                Ok(v)
            }
        }
    }
}
