use std::{
    convert::Infallible,
    future::Future,
    ops::{Deref, DerefMut},
};

use database_traits::get_connect::{
    Body, FromRequest, GetDatabaseConnect, GetDatabaseTransaction,
    RequestParts, TransactionOps,
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    StreamTrait, TransactionStream, TransactionTrait,
};

use crate::static_vars::{get_sql_database, get_sql_transaction};


#[derive(Debug, Default)]
pub struct SqlConnect;

impl FromRequest<Body> for SqlConnect {
    type Rejection = Infallible;

    fn from_request<'life0, 'async_trait>(
        _: &'life0 mut RequestParts<Body>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async { Ok(SqlConnect) })
    }
}

impl GetDatabaseConnect for SqlConnect {
    type Connect<'s> = DatabaseConnection;
    type Error = DbErr;

    fn get_connect(&self) -> Result<&Self::Connect<'_>, Self::Error> {
        Ok(get_sql_database())
    }
}

#[derive(Debug)]
pub struct SqlTransaction(pub DatabaseTransaction);

impl GetDatabaseTransaction for SqlConnect {
    type Transaction<'s> = SqlTransaction;

    type TransactionFuture<'s> =
        impl Future<Output = Result<SqlTransaction, DbErr>> + 's;

    fn get_transaction(&self) -> Self::TransactionFuture<'_> {
        async { get_sql_transaction().await.map(SqlTransaction) }
    }
}

impl TransactionOps for SqlTransaction {
    type Error = DbErr;

    type RollBackFuture<'r> = impl Future<Output = Result<(), DbErr>> + 'r;
    type SubmitFuture<'r> = impl Future<Output = Result<(), DbErr>> + 'r;

    fn submit<'s>(self) -> Self::SubmitFuture<'s>
    where
        Self: 's,
    {
        self.0.rollback()
    }

    fn roll_back<'r>(self) -> Self::RollBackFuture<'r>
    where
        Self: 'r,
    {
        self.0.commit()
    }
}

impl Deref for SqlTransaction {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for SqlTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl ConnectionTrait for SqlTransaction {
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self, stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<sea_orm::ExecResult, DbErr>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.execute(stmt)
    }

    fn query_one<'life0, 'async_trait>(
        &'life0 self, stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<Option<sea_orm::QueryResult>, DbErr>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.query_one(stmt)
    }

    fn query_all<'life0, 'async_trait>(
        &'life0 self, stmt: sea_orm::Statement,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<Vec<sea_orm::QueryResult>, DbErr>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.query_all(stmt)
    }
}

impl<'a> StreamTrait<'a> for SqlTransaction {
    type Stream = TransactionStream<'a>;

    fn stream(
        &'a self, stmt: sea_orm::Statement,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Stream, DbErr>> + 'a + Send>,
    > {
        self.0.stream(stmt)
    }
}

impl TransactionTrait for SqlTransaction {
    fn begin<'life0, 'async_trait>(
        &'life0 self,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<DatabaseTransaction, DbErr>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.begin()
    }

    fn transaction<'life0, 'async_trait, F, T, E>(
        &'life0 self, callback: F,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = Result<T, sea_orm::TransactionError<E>>,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        F: for<'c> FnOnce(
                &'c DatabaseTransaction,
            ) -> std::pin::Pin<
                Box<dyn Future<Output = Result<T, E>> + Send + 'c>,
            > + Send,
        T: Send,
        E: std::error::Error + Send,
        F: 'async_trait,
        T: 'async_trait,
        E: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.transaction(callback)
    }
}
