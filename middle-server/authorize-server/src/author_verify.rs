mod service;
mod layer;

use std::fmt::Debug;
use std::future::Future;
use axum::body::Body;
use axum_resp_result::RespError;
use http::{Request, Response};
use http::request::Parts;

pub use layer::AuthorizeLayer;
pub use service::UserAuthorize;

pub trait AuthorVerifier{
    type AuthorizedUser:Clone + Send +Sync +'static +Debug;

    type Error: RespError;

    type Future: Future<Output = Result<Self::AuthorizedUser, Self::Error>> +Send + 'static;


    fn authorize(&mut self, request_parts:Parts)->Self::Future;
    
}


