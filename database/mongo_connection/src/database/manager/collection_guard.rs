use std::{marker::PhantomData, ops::Deref};

use futures::Future;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::{MongoDbError, MongoErr};

pub struct CollectionGuard<C>
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
{
    pub(super) inner: Collection<C>,
}

impl<C> CollectionGuard<C>
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
{
    pub async fn doing<'s, F, Fut, O>(
        &'s self, handle: F,
    ) -> Result<O, MongoDbError>
    where
        F: FnOnce(&'s Collection<C>) -> Fut + 's,
        Fut: Future<Output = Result<O, MongoErr>> + 's,
    {
        handle(&self.inner).await.map_err(Into::into)
    }

    pub fn with_mapping<
        's,
        M: Serialize + for<'de> Deserialize<'de> + 'static,
    >(
        &'s self,
    ) -> CollectionMapping<'s, C, M> {
        CollectionMapping {
            _pha: PhantomData,
            inner: CollectionGuard {
                inner: self.inner.clone_with_type::<M>(),
            },
        }
    }
}

impl<C> Deref for CollectionGuard<C>
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
{
    type Target = Collection<C>;

    fn deref(&self) -> &Self::Target { &self.inner }
}

pub struct CollectionMapping<'s, C, M>
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
    M: Serialize + for<'de> Deserialize<'de> + 'static,
{
    _pha: PhantomData<&'s C>,
    inner: CollectionGuard<M>,
}

impl<'s, C, M> Deref for CollectionMapping<'s, C, M>
where
    C: Serialize + for<'de> Deserialize<'de> + 'static,
    M: Serialize + for<'de> Deserialize<'de> + 'static,
{
    type Target = CollectionGuard<M>;

    fn deref(&self) -> &Self::Target { &self.inner }
}
