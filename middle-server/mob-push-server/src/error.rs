use std::fmt::{Debug, Display};

use general_request_client::traits::RequestBuilder;

/// mob push 推送期间的异常
pub enum MobPushError<C>
where
    C: RequestBuilder,
{
    /// 发起请求时异常
    Request(C::Error),
    /// json 序列化、反序列化异常
    Json(serde_json::Error),
    /// mob 推送响应异常
    Mob { state: u16, msg: String },
}

impl<C> std::fmt::Debug for MobPushError<C>
where
    C: RequestBuilder,
    C::Error: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(err) => {
                f.debug_tuple("Request").field(err).finish()
            }
            MobPushError::Mob { state, msg } => f
                .debug_struct("Mob")
                .field("state", state)
                .field("msg", msg)
                .finish(),
            MobPushError::Json(err) => {
                f.debug_tuple("Json").field(err).finish()
            }
        }
    }
}

impl<C> std::fmt::Display for MobPushError<C>
where
    C: RequestBuilder,
    C::Error: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
       
            MobPushError::Request(err) => write!(f, "Request Error : {err}"),
            MobPushError::Mob { state, msg } => {
                write!(f, "Mob Pusher Error : [{}] {}", state, msg)
            }
            MobPushError::Json(err) => write!(f, "Json Error : {err}"),
        }
    }
}

impl<C> std::error::Error for MobPushError<C>
where
    C: RequestBuilder,
    C::Error: std::error::Error,
{
}

impl<C: RequestBuilder> From<(u16, String)> for MobPushError<C> {
    fn from((state, msg): (u16, String)) -> Self {
        Self::Mob { state, msg }
    }
}

impl<C: RequestBuilder> From<serde_json::Error> for MobPushError<C> {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
