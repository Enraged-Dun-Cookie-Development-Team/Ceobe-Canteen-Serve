use std::fmt::Debug;


/// mob push 推送期间的异常
#[derive(Debug, thiserror::Error)]
pub enum MobPushError {
    /// 发起请求时异常
    #[error("发起推送请求异常: {0}")]
    Request(#[from] general_request_client::Error),
    /// json 序列化、反序列化异常
    #[error(transparent)]
    Json(#[from]serde_json::Error),
    /// mob 推送响应异常
    #[error("MobPush 响应异常: {state} => `{msg}`")]
    Mob { state: u16, msg: String },
}

