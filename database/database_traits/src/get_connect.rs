use std::{error::Error as StdError, future::Future};

pub use axum::{
    body::Body,
    extract::{FromRequest, RequestParts},
};

pub trait GetDatabaseConnect: FromRequest<Body> {
    type Error: std::error::Error;
    type Connect<'s>: 's
    where
        Self: 's;

    fn get_connect(&self) -> Result<&Self::Connect<'_>, Self::Error>;
}

pub trait GetDatabaseConnectGuard: FromRequest<Body> {
    type Error: std::error::Error;
    type ConnectGuard<'s>: 's
    where
        Self: 's;

    fn get_connect_guard(&self) -> Result<Self::ConnectGuard<'_>, Self::Error>;
}

pub trait GetDatabaseCollection<C>: GetDatabaseConnect {
    type CollectGuard<'s>: 's
    where
        Self: 's;
    fn get_collection(&self) -> Result<Self::CollectGuard<'_>, Self::Error>;
}

pub trait GetDatabaseTransaction: GetDatabaseConnect {
    type Transaction<'s>: TransactionOps<Error = Self::Error> + 's
    where
        Self: 's;

    type TransactionFuture<'s>: Future<Output = Result<Self::Transaction<'s>, Self::Error>>
        + Send
        + 's
    where
        Self: 's;

    fn get_transaction(&self) -> Self::TransactionFuture<'_>;
}
pub trait TransactionOps {
    type Error: StdError;

    type SubmitFuture<'s>: Future<Output = Result<(), Self::Error>>
        + 's
        + Send
    where
        Self: 's;

    fn submit<'s>(self) -> Self::SubmitFuture<'s>
    where
        Self: 's;

    type RollBackFuture<'r>: Future<Output = Result<(), Self::Error>>
        + 'r
        + Send
    where
        Self: 'r;

    fn roll_back<'r>(self) -> Self::RollBackFuture<'r>
    where
        Self: 'r;
}
