use core::{future::Future, marker::Send};
use std::convert::Infallible;

use axum_core::extract::FromRequestParts;
use futures::future::ok;
use sql_models::sql_connection::database_traits::get_connect::Parts;

pub mod datasource;
pub mod detail;
pub mod global;
pub mod platform;

#[derive(Debug)]
pub struct FetcherConfigLogic;

impl<S> FromRequestParts<S> for FetcherConfigLogic {
    type Rejection = Infallible;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _: &'life0 mut Parts, _: &'life1 S,
    ) -> core::pin::Pin<
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
        Box::pin(ok(FetcherConfigLogic))
    }
}

pub trait SubLogic {
    type Parent;
    fn from_parent(parent: &Self::Parent) -> &Self;
}

pub trait SuperLogic {
    fn sub_logic<S: SubLogic<Parent = Self>>(&self) -> &S {
        S::from_parent(self)
    }
}

impl<S> SuperLogic for S {}

pub struct DetailConfig;

impl SubLogic for DetailConfig {
    type Parent = FetcherConfigLogic;

    fn from_parent(_: &Self::Parent) -> &Self { &DetailConfig }
}
pub struct DatasourceConfig;

impl SubLogic for DatasourceConfig {
    type Parent = FetcherConfigLogic;

    fn from_parent(_: &Self::Parent) -> &Self { &DatasourceConfig }
}
pub struct PlatformConfig;

impl SubLogic for PlatformConfig {
    type Parent = FetcherConfigLogic;

    fn from_parent(_parent: &Self::Parent) -> &Self { &PlatformConfig }
}
pub struct GlobalConfig;

impl SubLogic for GlobalConfig {
    type Parent = FetcherConfigLogic;

    fn from_parent(_parent: &Self::Parent) -> &Self { &GlobalConfig }
}
