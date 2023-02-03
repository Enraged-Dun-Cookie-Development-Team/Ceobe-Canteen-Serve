use axum_core::extract::FromRequestParts;
use futures::TryFutureExt;

use super::DatabaseOperate;
use crate::get_connect::{
    GetDatabaseCollection, GetDatabaseConnect, GetDatabaseTransaction,
    GetMutDatabaseConnect,
};

impl<C> GetMutDatabaseConnect for DatabaseOperate<C>
where
    C: GetMutDatabaseConnect,
{
    type Connect = C::Connect;

    fn mut_connect(&mut self) -> &mut Self::Connect {
        self.connect.mut_connect()
    }
}

impl<C> GetDatabaseConnect for DatabaseOperate<C>
where
    C: GetDatabaseConnect,
{
    type Connect = C::Connect;

    fn get_connect(&self) -> &Self::Connect { self.connect.get_connect() }
}

impl<C> GetDatabaseTransaction for DatabaseOperate<C>
where
    C: GetDatabaseTransaction,
{
    type Error = C::Error;
    type Transaction<'s> = C::Transaction<'s>
    where
        Self: 's;
    type TransactionFuture<'s> = C::TransactionFuture<'s>
    where
        Self: 's;

    fn get_transaction(&self) -> Self::TransactionFuture<'_> {
        self.connect.get_transaction()
    }
}

impl<C, Collect> GetDatabaseCollection<Collect> for DatabaseOperate<C>
where
    C: GetDatabaseCollection<Collect>,
{
    type CollectGuard<'s> =C::CollectGuard<'s>
    where
        Self: 's;
    type Error = C::Error;

    fn get_collection(&self) -> Result<Self::CollectGuard<'_>, Self::Error> {
        self.connect.get_collection()
    }
}

impl<C, S> FromRequestParts<S> for DatabaseOperate<C>
where
    C: FromRequestParts<S>,
{
    type Rejection = C::Rejection;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut http::request::Parts, state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(
            C::from_request_parts(parts, state)
                .map_ok(|connect| DatabaseOperate { connect }),
        )
    }
}
