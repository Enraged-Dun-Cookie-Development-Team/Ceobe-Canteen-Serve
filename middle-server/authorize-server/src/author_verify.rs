use std::{fmt::Debug, future::Future};

use axum_resp_result::RespError;
use http::request::Parts;
pub use layer::AuthorizeLayer;
pub use service::UserAuthorize;

mod layer;
mod service;

pub trait AuthorVerifier {
    type AuthorizedUser: Clone + Send + Sync + 'static + Debug;

    type Error: RespError;

    type Future: Future<Output = Result<Self::AuthorizedUser, Self::Error>>
        + Send
        + 'static;

    fn authorize(&mut self, request_parts: Parts) -> Self::Future;
}
