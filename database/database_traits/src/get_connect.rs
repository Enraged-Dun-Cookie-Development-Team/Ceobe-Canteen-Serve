use std::{error::Error as StdError, future::Future};

pub trait GetDatabaseConnect {
    type Error: std::error::Error;
    type Connect<'s>: 's
    where
        Self: 's;

    fn get_connect(&self) -> Result<Self::Connect<'_>, Self::Error>;
}

pub trait GetDatabaseTransaction {
    type Error: StdError;

    type Transaction<'s>: TransactionOps+ 's
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
