use std::{error::Error as StdError, future::Future};

pub use axum_core::extract::FromRequestParts;
pub use http::request::Parts;

/// 获取数据库连接， 以及后续操作基本trait
pub trait GetDatabaseConnect {
    /// 数据库连接本身
    type Connect<'s>: 's
    where
        Self: 's;

    /// 获取一个数据库连接
    /// # Note
    /// 获取一个连接引用不应有任何错误发生
    fn get_connect(&self) -> &Self::Connect<'_>;
}

/// 获取数据库连接， 以及后续操作基本trait
/// 获取的为 `&mut`
pub trait GetMutDatabaseConnect {
    /// 数据库连接本身
    type Connect<'s>: 's
    where
        Self: 's;
    /// 获取一个数据库连接
    /// # Note
    /// 获取一个连接引用不应有任何错误发生
    fn mut_connect(&mut self) -> &mut Self::Connect<'_>;
}

pub trait GetDatabaseCollection<C>: GetDatabaseConnect {
    /// 获取Collection 期间可能的异常
    type Error: StdError;
    type CollectGuard<'s>: 's
    where
        Self: 's;
    fn get_collection(&self) -> Result<Self::CollectGuard<'_>, Self::Error>;
}

pub trait GetDatabaseTransaction: GetDatabaseConnect {
    /// 建立transaction时可能的异常
    type Error: StdError;
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
