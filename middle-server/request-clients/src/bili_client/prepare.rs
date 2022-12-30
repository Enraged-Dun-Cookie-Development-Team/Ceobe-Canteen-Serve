use axum_starter::{prepare, state::AddState};
use tokio::{spawn, sync::mpsc};
use url::Url;

use super::{idle_wroker::idle_client, sender::QueryBiliVideo};

#[prepare(box BiliClientPrepare?)]
pub fn prepare_bili_client() -> Result<AddState<QueryBiliVideo>, PrepareError>
{
    // 创建 client
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) \
             Gecko/20100101 Firefox/102.0",
        )
        .build()?;

    // base url
    let base_url =
        Url::parse("https://api.bilibili.com/x/web-interface/view")?;
    // mpsc 管道
    let (send, recv) = mpsc::channel(8);
    // 启动独立协程处理请求
    spawn(idle_client(base_url, client, recv));
    // 注册到State
    Ok(AddState::new(QueryBiliVideo::new(send)))
}
#[derive(Debug, thiserror::Error)]
pub enum PrepareError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}
