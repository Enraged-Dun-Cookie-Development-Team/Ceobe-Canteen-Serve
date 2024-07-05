use std::{future::Future, pin::Pin};

pub mod database_operates;
pub mod get_connect;
pub mod initial;

pub type BoxedResultFuture<'r, T, E> =
    Pin<Box<dyn Future<Output = Result<T, E>> + 'r>>;
pub type BoxedResultSendFuture<'r, T, E> =
    Pin<Box<dyn Future<Output = Result<T, E>> + 'r + Send>>;

pub use database_operates::{
    operate_trait::OperateTrait,
    sub_operate::{SubOperate, SuperOperate},
};
pub use paste::paste;
