use core::{future::Future, marker::Send, pin::Pin};
use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

use database_traits::{
    get_connect::{
        FromRequestParts, GetDatabaseConnect, GetDatabaseTransaction,
        Parts, TransactionOps,
    },
    BoxedResultSendFuture,
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr,
    StreamTrait, TransactionStream, TransactionTrait,
};

use crate::static_vars::{get_sql_database, get_sql_transaction};

#[derive(Debug, Default)]
pub struct SqlConnect;

impl<S> FromRequestParts<S> for SqlConnect {
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut Parts, _state: &'life1 S,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Self, Self::Rejection>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
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
        BoxedResultSendFuture<'s, SqlTransaction, DbErr>;

    fn get_transaction(&self) -> Self::TransactionFuture<'_> {
        Box::pin(async { get_sql_transaction().await.map(SqlTransaction) })
    }
}

impl TransactionOps for SqlTransaction {
    type Error = DbErr;
    type RollBackFuture<'r> = BoxedResultSendFuture<'r, (), DbErr>;
    type SubmitFuture<'r> = BoxedResultSendFuture<'r, (), DbErr>;

    fn submit<'s>(self) -> Self::SubmitFuture<'s>
    where
        Self: 's,
    {
        Box::pin(self.0.commit())
    }

    fn roll_back<'r>(self) -> Self::RollBackFuture<'r>
    where
        Self: 'r,
    {
        Box::pin(self.0.rollback())
    }
}

impl Deref for SqlTransaction {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SqlTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ConnectionTrait for SqlTransaction {
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self, stmt: sea_orm::Statement,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<sea_orm::ExecResult, DbErr>>
                + Send
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
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Option<sea_orm::QueryResult>, DbErr>>
                + Send
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
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<sea_orm::QueryResult>, DbErr>>
                + Send
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

impl StreamTrait for SqlTransaction {
    type Stream<'a> = TransactionStream<'a>;

    fn stream<'a>(
        &'a self, stmt: sea_orm::Statement,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Stream<'a>, DbErr>> + 'a + Send>,
    > {
        self.0.stream(stmt)
    }
}

impl TransactionTrait for SqlTransaction {
    fn begin<'life0, 'async_trait>(
        &'life0 self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<DatabaseTransaction, DbErr>>
                + Send
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
    ) -> Pin<
        Box<
            dyn Future<Output = Result<T, sea_orm::TransactionError<E>>>
                + Send
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

    fn begin_with_config<'life0, 'async_trait>(
        &'life0 self, isolation_level: Option<sea_orm::IsolationLevel>,
        access_mode: Option<sea_orm::AccessMode>,
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
        self.0.begin_with_config(isolation_level, access_mode)
    }

    fn transaction_with_config<'life0, 'async_trait, F, T, E>(
        &'life0 self, callback: F,
        isolation_level: Option<sea_orm::IsolationLevel>,
        access_mode: Option<sea_orm::AccessMode>,
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
            ) -> Pin<
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
        self.0.transaction_with_config(callback, isolation_level, access_mode)
    }
}
