use std::ops::{Deref, DerefMut};

use db_ops_prelude::sea_orm::prelude::async_trait::async_trait;
use persistence::operate::{FromRequestParts, Parts};

pub mod error;
pub mod impletements;
pub mod release_version;
pub mod view;

pub struct CeobeOperationLogic<T>(T);

#[async_trait]
impl<T, S> FromRequestParts<S> for CeobeOperationLogic<T>
where
    T: FromRequestParts<S>,
    S: Send + Sync + 'static + Clone,
{
    type Rejection = <T as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts, state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(
            <T as FromRequestParts<S>>::from_request_parts(parts, state)
                .await?,
        ))
    }
}

impl<T> DerefMut for CeobeOperationLogic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T> Deref for CeobeOperationLogic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.0 }
}
